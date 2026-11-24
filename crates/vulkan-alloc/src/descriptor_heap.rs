//! Byte layout of `VK_EXT_descriptor_heap` heaps.
//!
//! A descriptor heap is a buffer. Where its descriptors sit, how far apart, and where the
//! range reserved for the implementation goes follows from
//! [`vk::PhysicalDeviceDescriptorHeapPropertiesEXT`]. This module computes that layout
//! and nothing else: it holds no Vulkan objects, allocates no memory and keeps no record
//! of which slots are in use.
//!
//! [`ResourceHeapPlan`] and [`SamplerHeapPlan`] place runs of descriptor slots
//! ([`DescriptorSlots`]) one after another, in the order they are requested, then the
//! reserved range, and produce a [`DescriptorHeapLayout`].
//!
//! ```
//! use vulkan::vk;
//! use vulkan_alloc::descriptor_heap::{HeapLayoutError, ResourceHeapPlan};
//!
//! # fn main() -> Result<(), HeapLayoutError> {
//! let properties = vk::PhysicalDeviceDescriptorHeapPropertiesEXT::default()
//!     .resource_heap_alignment(64)
//!     .max_resource_heap_size(1 << 20)
//!     .min_resource_heap_reserved_range(256)
//!     .buffer_descriptor_size(16)
//!     .buffer_descriptor_alignment(16)
//!     .image_descriptor_size(32)
//!     .image_descriptor_alignment(32);
//!
//! let mut plan = ResourceHeapPlan::new(&properties);
//! let buffers = plan.buffers(2)?;
//! let images = plan.images(16)?;
//! let layout = plan.finish()?;
//!
//! let mut heap = vec![0u8; layout.size as usize];
//! let destination = images.bytes(&mut heap, 3).expect("slot 3 is in the run");
//! assert_eq!(destination.len(), 32);
//!
//! assert_eq!(buffers.index(1), Some(1));
//! assert_eq!(images.index(3), Some(4));
//! assert_eq!(images.offset(3), Some(4 * 32));
//!
//! let bind = layout.bind_info(0x1_0000)?;
//! assert_eq!(bind.reserved_range_offset, layout.reserved_offset);
//! # Ok(())
//! # }
//! ```
//!
//! In practice the heap is a buffer of [`DescriptorHeapLayout::size`] bytes created with
//! `DESCRIPTOR_HEAP_EXT` usage and mapped, and the descriptors are written into the
//! slices that [`DescriptorSlots::bytes`] or [`DescriptorSlots::slices`] return.
//!
//! # Indices
//!
//! A shader reaches a descriptor through an index, and the byte offset of the descriptor
//! is `index * stride`. The first slot of a run is at `first_index * stride`, so
//! [`DescriptorSlots::index`] is the number to hand to the shader.
//!
//! - [`ResourceHeapPlan::buffers`], [`ResourceHeapPlan::images`] and
//!   [`SamplerHeapPlan::samplers`] make runs whose stride is the size of their
//!   descriptor type, the indexing used by shaders that reach descriptors through
//!   untyped pointers.
//! - [`ResourceHeapPlan::uniform`] makes a run whose slots can each hold an image or a
//!   buffer descriptor, for mappings such as [`vk::DescriptorSetAndBindingMappingEXT`]
//!   where the application chooses the stride (`heapIndexStride`).

use core::fmt;
use core::ops::Range;
use vulkan::vk;

/// Why a heap cannot be laid out or bound as asked.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeapLayoutError {
    /// The heap would be larger than `maxResourceHeapSize` or `maxSamplerHeapSize`.
    TooLarge {
        /// The size the heap would have, in bytes.
        size: u64,
        /// The largest size the device allows, in bytes.
        max: u64,
    },
    /// The address of the heap is not a multiple of [`DescriptorHeapLayout::alignment`].
    Misaligned {
        /// The address that was passed.
        address: u64,
        /// What the address must be a multiple of.
        alignment: u64,
    },
    /// An index does not fit in `u32`, or a byte offset, a size or the least common
    /// multiple of two descriptor alignments does not fit in `u64`.
    Overflow,
    /// The size of a descriptor type is not a multiple of its alignment, so descriptors
    /// indexed in units of their size cannot all be aligned.
    UnalignedDescriptorSize {
        /// The descriptor size, in bytes.
        size: u64,
        /// The descriptor alignment, in bytes.
        alignment: u64,
    },
    /// The size of a descriptor type is 0, so its descriptors cannot be indexed in units
    /// of their size. A device reports a power of two for every descriptor size; 0
    /// usually means the properties were not filled in by
    /// `vkGetPhysicalDeviceProperties2`.
    ZeroDescriptorSize,
    /// The reserved range of a [`DescriptorHeapLayout`] does not lie inside the heap:
    /// `reserved_offset + reserved_size` is larger than `size`.
    ReservedRangeOutsideHeap {
        /// [`DescriptorHeapLayout::reserved_offset`].
        reserved_offset: u64,
        /// [`DescriptorHeapLayout::reserved_size`].
        reserved_size: u64,
        /// [`DescriptorHeapLayout::size`].
        heap_size: u64,
    },
    /// The address of the heap is 0. A heap range must come from a buffer, and no buffer
    /// has device address 0, so this is rejected whatever the size of the heap.
    NullAddress,
    /// The heap would reach past the last device address: the address of its last
    /// byte, `address + size - 1`, does not fit in `u64`.
    AddressRangeOverflow {
        /// The address that was passed.
        address: u64,
        /// [`DescriptorHeapLayout::size`].
        size: u64,
    },
}

impl fmt::Display for HeapLayoutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::TooLarge { size, max } => write!(
                f,
                "descriptor heap needs {size} bytes, the device allows {max}"
            ),
            Self::Misaligned { address, alignment } => write!(
                f,
                "descriptor heap address {address:#x} is not a multiple of {alignment}"
            ),
            Self::Overflow => f.write_str("descriptor heap layout overflows"),
            Self::UnalignedDescriptorSize { size, alignment } => write!(
                f,
                "descriptor size {size} is not a multiple of its alignment {alignment}"
            ),
            Self::ZeroDescriptorSize => f.write_str("descriptor size is 0"),
            Self::ReservedRangeOutsideHeap {
                reserved_offset,
                reserved_size,
                heap_size,
            } => write!(
                f,
                "reserved range of {reserved_size} bytes at offset {reserved_offset} does not \
                 fit in a descriptor heap of {heap_size} bytes"
            ),
            Self::NullAddress => f.write_str("descriptor heap address is 0"),
            Self::AddressRangeOverflow { address, size } => write!(
                f,
                "descriptor heap of {size} bytes at {address:#x} reaches past the last \
                 device address"
            ),
        }
    }
}

impl core::error::Error for HeapLayoutError {}

/// A run of consecutive slots that hold one kind of descriptor.
///
/// ```
/// use vulkan_alloc::descriptor_heap::DescriptorSlots;
///
/// let slots = DescriptorSlots {
///     first_index: 2,
///     count: 3,
///     stride: 32,
///     descriptor_size: 24,
/// };
/// assert_eq!(slots.index(0), Some(2));
/// assert_eq!(slots.offset(1), Some(96));
/// assert_eq!(slots.range(1), Some(96..120));
/// assert_eq!(slots.index(3), None);
/// assert_eq!(slots.end(), 160);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DescriptorSlots {
    /// The index of the first slot, in units of `stride`.
    pub first_index: u32,
    /// The number of slots.
    pub count: u32,
    /// The distance in bytes from one slot to the next.
    pub stride: u64,
    /// The number of bytes one descriptor occupies, which is what
    /// `vkWriteResourceDescriptorsEXT` or `vkWriteSamplerDescriptorsEXT` writes. The
    /// plans make it at most `stride`.
    pub descriptor_size: u64,
}

impl DescriptorSlots {
    /// The index a shader uses for slot `slot` of this run, or `None` if `slot` is not
    /// below `count` or the index does not fit in `u32`.
    pub const fn index(&self, slot: u32) -> Option<u32> {
        if slot < self.count {
            self.first_index.checked_add(slot)
        } else {
            None
        }
    }

    /// The byte offset of slot `slot` from the start of the heap, or `None` if `slot` is
    /// not in the run or the offset does not fit in `u64`.
    pub const fn offset(&self, slot: u32) -> Option<u64> {
        match self.index(slot) {
            Some(index) => (index as u64).checked_mul(self.stride),
            None => None,
        }
    }

    /// The bytes of the heap that the descriptor of slot `slot` occupies:
    /// `descriptor_size` bytes from [`DescriptorSlots::offset`]. `None` if `slot` is not
    /// in the run or the range does not fit in `usize`.
    pub fn range(&self, slot: u32) -> Option<Range<usize>> {
        let start = usize::try_from(self.offset(slot)?).ok()?;
        let end = start.checked_add(usize::try_from(self.descriptor_size).ok()?)?;
        Some(start..end)
    }

    /// The bytes of slot `slot` inside the mapped heap `heap`, as given by
    /// [`DescriptorSlots::range`]. `None` if `slot` is not in the run or `heap` is too
    /// short.
    pub fn bytes<'a>(&self, heap: &'a mut [u8], slot: u32) -> Option<&'a mut [u8]> {
        heap.get_mut(self.range(slot)?)
    }

    /// The bytes of every slot of the run inside the mapped heap `heap`, in slot order.
    ///
    /// The slice of slot `slot` starts at [`DescriptorSlots::offset`] and is
    /// `descriptor_size` bytes long, or `stride` bytes long if `descriptor_size` is
    /// larger than `stride`. When `stride` is not 0 and `descriptor_size` is at most
    /// `stride`, as in every run the plans make, each slice is the one [`DescriptorSlots::bytes`] returns for its
    /// slot, and the result is `Some` exactly when `bytes` is `Some` for every slot of
    /// the run: `heap` has to reach the end of the last descriptor, not the end of its
    /// slot.
    ///
    /// The slices do not overlap, so they can all be destinations of one
    /// `vkWriteResourceDescriptorsEXT` or `vkWriteSamplerDescriptorsEXT` call that fills
    /// the whole run. A run of 0 slots gives no slices. `None` if `stride` is 0, or if
    /// the index of a slot does not fit in `u32`, its slice does not fit in `usize`, or
    /// `heap` is too short to hold its slice.
    pub fn slices<'a>(
        &self,
        heap: &'a mut [u8],
    ) -> Option<impl Iterator<Item = &'a mut [u8]> + use<'a>> {
        if self.stride == 0 {
            return None;
        }
        let size = self.descriptor_size.min(self.stride);
        let slice = |slot: u32| -> Option<Range<usize>> {
            let start = usize::try_from(self.offset(slot)?).ok()?;
            Some(start..start.checked_add(usize::try_from(size).ok()?)?)
        };
        let run = match self.count.checked_sub(1) {
            Some(last) => slice(0)?.start..slice(last)?.end,
            None => 0..0,
        };
        let mut rest = heap.get_mut(run)?;
        let gap = usize::try_from(self.stride - size).unwrap_or(usize::MAX);
        let size = usize::try_from(size).unwrap_or(usize::MAX);
        Some((0..self.count).map(move |_| {
            let (descriptor, after) = core::mem::take(&mut rest).split_at_mut(size);
            rest = after.get_mut(gap..).unwrap_or_default();
            descriptor
        }))
    }

    /// The byte offset just past the last slot, `(first_index + count) * stride`. It
    /// saturates at `u64::MAX` instead of overflowing.
    pub const fn end(&self) -> u64 {
        (self.first_index as u64 + self.count as u64).saturating_mul(self.stride)
    }
}

/// Size, alignment and reserved range of a heap, made by [`ResourceHeapPlan::finish`] or
/// [`SamplerHeapPlan::finish`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DescriptorHeapLayout {
    /// The size in bytes of the buffer to create. It ends with the reserved range.
    ///
    /// It is 0 when the plan holds no slots (no runs, or only runs of 0 slots) and the
    /// reserved range is 0 bytes; there is then no buffer to create and no heap to bind.
    pub size: u64,
    /// What the device address of the heap must be a multiple of:
    /// `resourceHeapAlignment` or `samplerHeapAlignment`, with 0 treated as 1.
    /// [`DescriptorHeapLayout::bind_info`] checks the address.
    pub alignment: u64,
    /// Where the range reserved for the implementation starts. Do not write descriptors
    /// there.
    pub reserved_offset: u64,
    /// The size of the reserved range: `minResourceHeapReservedRange`,
    /// `minSamplerHeapReservedRange`, or for a plan made by
    /// [`SamplerHeapPlan::with_embedded_samplers`] the larger of
    /// `minSamplerHeapReservedRange` and `minSamplerHeapReservedRangeWithEmbedded`.
    pub reserved_size: u64,
}

impl DescriptorHeapLayout {
    /// The [`vk::BindHeapInfoEXT`] for a heap of this layout at device address
    /// `address`, covering [`DescriptorHeapLayout::size`] bytes and naming the reserved
    /// range.
    ///
    /// It checks what the layout and `address` alone can tell. It cannot check that the
    /// range `address .. address + size` lies inside a buffer created with
    /// `DESCRIPTOR_HEAP_EXT` usage, that [`DescriptorHeapLayout::alignment`] is the
    /// alignment the device requires, that `size` and `reserved_size` are within the
    /// device limits, or that `reserved_offset` is a multiple of the descriptor
    /// alignments, which the layout does not store. [`ResourceHeapPlan::finish`] and
    /// [`SamplerHeapPlan::finish`] make layouts that meet the last three.
    ///
    /// # Errors
    ///
    /// The first of these that applies:
    ///
    /// - [`HeapLayoutError::ReservedRangeOutsideHeap`] if `reserved_offset +
    ///   reserved_size` is larger than `size`.
    /// - [`HeapLayoutError::Misaligned`] if `address` is not a multiple of
    ///   [`DescriptorHeapLayout::alignment`].
    /// - [`HeapLayoutError::NullAddress`] if `address` is 0, whatever `size` is.
    /// - [`HeapLayoutError::AddressRangeOverflow`] if the address of the last byte of
    ///   the heap, `address + size - 1`, does not fit in `u64`.
    pub fn bind_info(
        &self,
        address: vk::DeviceAddress,
    ) -> Result<vk::BindHeapInfoEXT<'static>, HeapLayoutError> {
        if self
            .reserved_offset
            .checked_add(self.reserved_size)
            .is_none_or(|end| end > self.size)
        {
            return Err(HeapLayoutError::ReservedRangeOutsideHeap {
                reserved_offset: self.reserved_offset,
                reserved_size: self.reserved_size,
                heap_size: self.size,
            });
        }
        if !address.is_multiple_of(self.alignment.max(1)) {
            return Err(HeapLayoutError::Misaligned {
                address,
                alignment: self.alignment,
            });
        }
        if address == 0 {
            return Err(HeapLayoutError::NullAddress);
        }
        if self
            .size
            .checked_sub(1)
            .is_some_and(|last| address.checked_add(last).is_none())
        {
            return Err(HeapLayoutError::AddressRangeOverflow {
                address,
                size: self.size,
            });
        }
        Ok(vk::BindHeapInfoEXT::default()
            .heap_range(
                vk::DeviceAddressRangeKHR::default()
                    .address(address)
                    .size(self.size),
            )
            .reserved_range_offset(self.reserved_offset)
            .reserved_range_size(self.reserved_size))
    }
}

#[derive(Debug, Clone)]
struct Plan {
    cursor: u64,
    reserved_alignment: Option<u64>,
    reserved_size: u64,
    heap_alignment: u64,
    max_size: u64,
}

impl Plan {
    fn run(
        &mut self,
        count: u32,
        stride: u64,
        descriptor_size: u64,
        alignment: u64,
    ) -> Result<DescriptorSlots, HeapLayoutError> {
        if stride == 0 {
            return Err(HeapLayoutError::ZeroDescriptorSize);
        }
        let alignment = alignment.max(1);
        if !stride.is_multiple_of(alignment) {
            return Err(HeapLayoutError::UnalignedDescriptorSize {
                size: stride,
                alignment,
            });
        }
        let first_index =
            u32::try_from(self.cursor.div_ceil(stride)).map_err(|_| HeapLayoutError::Overflow)?;
        let slots = DescriptorSlots {
            first_index,
            count,
            stride,
            descriptor_size,
        };
        if let Some(last_slot) = count.checked_sub(1) {
            first_index
                .checked_add(last_slot)
                .ok_or(HeapLayoutError::Overflow)?;
        }
        self.cursor = (u64::from(first_index) + u64::from(count))
            .checked_mul(stride)
            .ok_or(HeapLayoutError::Overflow)?;
        Ok(slots)
    }

    fn finish(self) -> Result<DescriptorHeapLayout, HeapLayoutError> {
        let reserved_alignment = self.reserved_alignment.ok_or(HeapLayoutError::Overflow)?;
        let reserved_offset = self
            .cursor
            .checked_next_multiple_of(reserved_alignment.max(1))
            .ok_or(HeapLayoutError::Overflow)?;
        let size = reserved_offset
            .checked_add(self.reserved_size)
            .ok_or(HeapLayoutError::Overflow)?;
        if size > self.max_size {
            return Err(HeapLayoutError::TooLarge {
                size,
                max: self.max_size,
            });
        }
        Ok(DescriptorHeapLayout {
            size,
            alignment: self.heap_alignment.max(1),
            reserved_offset,
            reserved_size: self.reserved_size,
        })
    }
}

fn lcm(a: u64, b: u64) -> Option<u64> {
    fn gcd(mut a: u64, mut b: u64) -> u64 {
        while b != 0 {
            (a, b) = (b, a % b);
        }
        a
    }
    let (a, b) = (a.max(1), b.max(1));
    (a / gcd(a, b)).checked_mul(b)
}

/// Plans the layout of a resource heap, which holds buffer and image descriptors.
///
/// Each run starts at the first multiple of its stride at or after the end of the
/// previous run, so runs never overlap but gaps can appear between runs of different
/// strides. A run of 0 slots is placed the same way: it has no slots, but it moves the
/// end of the plan up to that multiple of its stride, and the next run starts there or
/// later. A method that returns an error leaves the plan unchanged.
///
/// ```
/// use vulkan::vk;
/// use vulkan_alloc::descriptor_heap::{HeapLayoutError, ResourceHeapPlan};
///
/// # fn main() -> Result<(), HeapLayoutError> {
/// let properties = vk::PhysicalDeviceDescriptorHeapPropertiesEXT::default()
///     .max_resource_heap_size(1 << 20)
///     .buffer_descriptor_size(16)
///     .buffer_descriptor_alignment(16)
///     .image_descriptor_size(32)
///     .image_descriptor_alignment(32);
///
/// let mut plan = ResourceHeapPlan::new(&properties);
/// plan.buffers(1)?;
/// plan.images(0)?;
/// assert_eq!(plan.buffers(1)?.first_index, 2);
/// assert_eq!(plan.finish()?.size, 64);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct ResourceHeapPlan {
    plan: Plan,
    buffer_size: u64,
    buffer_alignment: u64,
    image_size: u64,
    image_alignment: u64,
}

impl ResourceHeapPlan {
    /// Starts an empty plan from the device's descriptor heap properties: the buffer
    /// and image descriptor sizes and alignments, `resourceHeapAlignment`,
    /// `maxResourceHeapSize` and `minResourceHeapReservedRange`. A descriptor alignment
    /// of 0 is treated as 1.
    pub fn new(properties: &vk::PhysicalDeviceDescriptorHeapPropertiesEXT<'_>) -> Self {
        Self {
            plan: Plan {
                cursor: 0,
                reserved_alignment: lcm(
                    properties.buffer_descriptor_alignment,
                    properties.image_descriptor_alignment,
                ),
                reserved_size: properties.min_resource_heap_reserved_range,
                heap_alignment: properties.resource_heap_alignment,
                max_size: properties.max_resource_heap_size,
            },
            buffer_size: properties.buffer_descriptor_size,
            buffer_alignment: properties.buffer_descriptor_alignment,
            image_size: properties.image_descriptor_size,
            image_alignment: properties.image_descriptor_alignment,
        }
    }

    /// Adds a run of `count` buffer descriptors, indexed in units of
    /// `bufferDescriptorSize`.
    ///
    /// # Errors
    ///
    /// The first of these that applies:
    ///
    /// - [`HeapLayoutError::ZeroDescriptorSize`] if `bufferDescriptorSize` is 0.
    /// - [`HeapLayoutError::UnalignedDescriptorSize`] if `bufferDescriptorSize` is not a
    ///   multiple of `bufferDescriptorAlignment`.
    /// - [`HeapLayoutError::Overflow`] if the first index of the run, or the index of one
    ///   of its slots, does not fit in `u32`, or its end does not fit in `u64`. A run of
    ///   0 slots has a first index too: where it starts, in units of the stride.
    pub fn buffers(&mut self, count: u32) -> Result<DescriptorSlots, HeapLayoutError> {
        self.plan.run(
            count,
            self.buffer_size,
            self.buffer_size,
            self.buffer_alignment,
        )
    }

    /// Adds a run of `count` image descriptors, indexed in units of
    /// `imageDescriptorSize`.
    ///
    /// # Errors
    ///
    /// The first of these that applies:
    ///
    /// - [`HeapLayoutError::ZeroDescriptorSize`] if `imageDescriptorSize` is 0.
    /// - [`HeapLayoutError::UnalignedDescriptorSize`] if `imageDescriptorSize` is not a
    ///   multiple of `imageDescriptorAlignment`.
    /// - [`HeapLayoutError::Overflow`] if the first index of the run, or the index of one
    ///   of its slots, does not fit in `u32`, or its end does not fit in `u64`. A run of
    ///   0 slots has a first index too: where it starts, in units of the stride.
    pub fn images(&mut self, count: u32) -> Result<DescriptorSlots, HeapLayoutError> {
        self.plan.run(
            count,
            self.image_size,
            self.image_size,
            self.image_alignment,
        )
    }

    /// Adds a run of `count` slots that can each hold an image descriptor or a buffer
    /// descriptor.
    ///
    /// The stride is the larger of the two descriptor sizes, rounded up to a multiple of
    /// both descriptor alignments; pass it as the index stride of the mapping. The
    /// `descriptor_size` of the run is the larger size; a buffer descriptor written into
    /// a slot uses a prefix of it.
    ///
    /// # Errors
    ///
    /// The first of these that applies:
    ///
    /// - [`HeapLayoutError::ZeroDescriptorSize`] if `bufferDescriptorSize` or
    ///   `imageDescriptorSize` is 0.
    /// - [`HeapLayoutError::Overflow`] if the least common multiple of the two
    ///   descriptor alignments or the stride does not fit in `u64`, the first index of
    ///   the run or the index of one of its slots does not fit in `u32`, or its end does
    ///   not fit in `u64`. A run of 0 slots has a first index too: where it starts, in
    ///   units of the stride.
    pub fn uniform(&mut self, count: u32) -> Result<DescriptorSlots, HeapLayoutError> {
        if self.buffer_size == 0 || self.image_size == 0 {
            return Err(HeapLayoutError::ZeroDescriptorSize);
        }
        let alignment =
            lcm(self.buffer_alignment, self.image_alignment).ok_or(HeapLayoutError::Overflow)?;
        let size = self.buffer_size.max(self.image_size);
        let stride = size
            .checked_next_multiple_of(alignment)
            .ok_or(HeapLayoutError::Overflow)?;
        self.plan.run(count, stride, size, alignment)
    }

    /// Places the reserved range at the smallest multiple of both descriptor alignments
    /// that is at or after the end of the last run, or at offset 0 if the plan has no
    /// runs, and returns the layout of the heap.
    ///
    /// When the plan holds no slots (no runs, or only runs of 0 slots) and
    /// `minResourceHeapReservedRange` is 0, the layout has a size of 0: there is no buffer
    /// to create and no heap to bind.
    ///
    /// # Errors
    ///
    /// The first of these that applies:
    ///
    /// - [`HeapLayoutError::Overflow`] if the least common multiple of the two
    ///   descriptor alignments, the offset of the reserved range or the size of the heap
    ///   does not fit in `u64`.
    /// - [`HeapLayoutError::TooLarge`] if the heap is larger than `maxResourceHeapSize`.
    pub fn finish(self) -> Result<DescriptorHeapLayout, HeapLayoutError> {
        self.plan.finish()
    }
}

/// Plans the layout of a sampler heap.
///
/// Runs, including runs of 0 slots, are placed one after another as in
/// [`ResourceHeapPlan`]. A method that returns an error leaves the plan unchanged.
#[derive(Debug, Clone)]
pub struct SamplerHeapPlan {
    plan: Plan,
    sampler_size: u64,
    sampler_alignment: u64,
}

impl SamplerHeapPlan {
    /// Starts an empty plan from the device's descriptor heap properties: the sampler
    /// descriptor size and alignment, `samplerHeapAlignment`, `maxSamplerHeapSize` and
    /// `minSamplerHeapReservedRange`. A sampler descriptor alignment of 0 is treated as
    /// 1.
    ///
    /// This reserved range can be too small while a shader uses embedded samplers; plan
    /// a heap bound for such shaders with [`SamplerHeapPlan::with_embedded_samplers`].
    pub fn new(properties: &vk::PhysicalDeviceDescriptorHeapPropertiesEXT<'_>) -> Self {
        Self {
            plan: Plan {
                cursor: 0,
                reserved_alignment: Some(properties.sampler_descriptor_alignment),
                reserved_size: properties.min_sampler_heap_reserved_range,
                heap_alignment: properties.sampler_heap_alignment,
                max_size: properties.max_sampler_heap_size,
            },
            sampler_size: properties.sampler_descriptor_size,
            sampler_alignment: properties.sampler_descriptor_alignment,
        }
    }

    /// Starts an empty plan as [`SamplerHeapPlan::new`] does, for a heap that is bound
    /// while a shader uses an embedded sampler through a descriptor mapping. The reserved
    /// range is the larger of `minSamplerHeapReservedRange` and
    /// `minSamplerHeapReservedRangeWithEmbedded`.
    ///
    /// The implementation may store the embedded sampler descriptors in the reserved
    /// range of the bound sampler heap, so while any bound shader uses an embedded
    /// sampler through a descriptor mapping (a mapping whose `pEmbeddedSampler` is not
    /// null), the specification requires the reserved range size of the sampler heap to
    /// be at least `minSamplerHeapReservedRangeWithEmbedded`. It must also be at least
    /// `minSamplerHeapReservedRange` whenever the heap is bound.
    ///
    /// ```
    /// use vulkan::vk;
    /// use vulkan_alloc::descriptor_heap::{HeapLayoutError, SamplerHeapPlan};
    ///
    /// # fn main() -> Result<(), HeapLayoutError> {
    /// let properties = vk::PhysicalDeviceDescriptorHeapPropertiesEXT::default()
    ///     .sampler_heap_alignment(64)
    ///     .max_sampler_heap_size(1 << 20)
    ///     .min_sampler_heap_reserved_range(96 * 32)
    ///     .min_sampler_heap_reserved_range_with_embedded(2048 * 32)
    ///     .sampler_descriptor_size(32)
    ///     .sampler_descriptor_alignment(32);
    ///
    /// let mut plan = SamplerHeapPlan::with_embedded_samplers(&properties);
    /// plan.samplers(16)?;
    /// let layout = plan.finish()?;
    /// assert_eq!(layout.reserved_offset, 16 * 32);
    /// assert_eq!(layout.reserved_size, 2048 * 32);
    /// assert_eq!(layout.size, 16 * 32 + 2048 * 32);
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_embedded_samplers(
        properties: &vk::PhysicalDeviceDescriptorHeapPropertiesEXT<'_>,
    ) -> Self {
        let mut plan = Self::new(properties);
        plan.plan.reserved_size = plan
            .plan
            .reserved_size
            .max(properties.min_sampler_heap_reserved_range_with_embedded);
        plan
    }

    /// Adds a run of `count` sampler descriptors, indexed in units of
    /// `samplerDescriptorSize`.
    ///
    /// # Errors
    ///
    /// The first of these that applies:
    ///
    /// - [`HeapLayoutError::ZeroDescriptorSize`] if `samplerDescriptorSize` is 0.
    /// - [`HeapLayoutError::UnalignedDescriptorSize`] if `samplerDescriptorSize` is not
    ///   a multiple of `samplerDescriptorAlignment`.
    /// - [`HeapLayoutError::Overflow`] if the first index of the run, or the index of one
    ///   of its slots, does not fit in `u32`, or its end does not fit in `u64`. A run of
    ///   0 slots has a first index too: where it starts, in units of the stride.
    pub fn samplers(&mut self, count: u32) -> Result<DescriptorSlots, HeapLayoutError> {
        self.plan.run(
            count,
            self.sampler_size,
            self.sampler_size,
            self.sampler_alignment,
        )
    }

    /// Places the reserved range at the smallest multiple of
    /// `samplerDescriptorAlignment` that is at or after the end of the last run, or at
    /// offset 0 if the plan has no runs, and returns the layout of the heap.
    ///
    /// When the plan holds no slots (no runs, or only runs of 0 slots) and the reserved
    /// range is 0 bytes, the layout has a size of 0: there is no buffer to create and no
    /// heap to bind.
    ///
    /// # Errors
    ///
    /// The first of these that applies:
    ///
    /// - [`HeapLayoutError::Overflow`] if the offset of the reserved range or the size
    ///   of the heap does not fit in `u64`.
    /// - [`HeapLayoutError::TooLarge`] if the heap is larger than `maxSamplerHeapSize`.
    pub fn finish(self) -> Result<DescriptorHeapLayout, HeapLayoutError> {
        self.plan.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use proptest::strategy::ValueTree;
    use proptest::test_runner::{RngAlgorithm, TestRng, TestRunner};

    const U32_MAX: u128 = u32::MAX as u128;
    const U64_MAX: u128 = u64::MAX as u128;
    const USIZE_MAX: u128 = usize::MAX as u128;
    const ADDRESS_SPACE: u128 = 1 << 64;
    const HEAP_BYTES: usize = 1 << 16;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct Properties {
        buffer_size: u64,
        buffer_alignment: u64,
        image_size: u64,
        image_alignment: u64,
        sampler_size: u64,
        sampler_alignment: u64,
        resource_heap_alignment: u64,
        sampler_heap_alignment: u64,
        max_resource_heap_size: u64,
        max_sampler_heap_size: u64,
        min_resource_reserved: u64,
        min_sampler_reserved: u64,
        min_sampler_reserved_with_embedded: u64,
    }

    impl Properties {
        fn fields_mut(&mut self) -> [&mut u64; 13] {
            [
                &mut self.buffer_size,
                &mut self.buffer_alignment,
                &mut self.image_size,
                &mut self.image_alignment,
                &mut self.sampler_size,
                &mut self.sampler_alignment,
                &mut self.resource_heap_alignment,
                &mut self.sampler_heap_alignment,
                &mut self.max_resource_heap_size,
                &mut self.max_sampler_heap_size,
                &mut self.min_resource_reserved,
                &mut self.min_sampler_reserved,
                &mut self.min_sampler_reserved_with_embedded,
            ]
        }

        fn vk(&self) -> vk::PhysicalDeviceDescriptorHeapPropertiesEXT<'static> {
            vk::PhysicalDeviceDescriptorHeapPropertiesEXT::default()
                .buffer_descriptor_size(self.buffer_size)
                .buffer_descriptor_alignment(self.buffer_alignment)
                .image_descriptor_size(self.image_size)
                .image_descriptor_alignment(self.image_alignment)
                .sampler_descriptor_size(self.sampler_size)
                .sampler_descriptor_alignment(self.sampler_alignment)
                .resource_heap_alignment(self.resource_heap_alignment)
                .sampler_heap_alignment(self.sampler_heap_alignment)
                .max_resource_heap_size(self.max_resource_heap_size)
                .max_sampler_heap_size(self.max_sampler_heap_size)
                .min_resource_heap_reserved_range(self.min_resource_reserved)
                .min_sampler_heap_reserved_range(self.min_sampler_reserved)
                .min_sampler_heap_reserved_range_with_embedded(
                    self.min_sampler_reserved_with_embedded,
                )
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Heap {
        Resource,
        Sampler,
        SamplerWithEmbedded,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Op {
        Buffers(u32),
        Images(u32),
        Uniform(u32),
        Samplers(u32),
    }

    enum Plan {
        Resource(ResourceHeapPlan),
        Sampler(SamplerHeapPlan),
    }

    impl Plan {
        fn new(properties: &Properties, heap: Heap) -> Self {
            let properties = properties.vk();
            match heap {
                Heap::Resource => Self::Resource(ResourceHeapPlan::new(&properties)),
                Heap::Sampler => Self::Sampler(SamplerHeapPlan::new(&properties)),
                Heap::SamplerWithEmbedded => {
                    Self::Sampler(SamplerHeapPlan::with_embedded_samplers(&properties))
                }
            }
        }

        fn apply(&mut self, op: Op) -> Result<DescriptorSlots, HeapLayoutError> {
            match (self, op) {
                (Self::Resource(plan), Op::Buffers(count)) => plan.buffers(count),
                (Self::Resource(plan), Op::Images(count)) => plan.images(count),
                (Self::Resource(plan), Op::Uniform(count)) => plan.uniform(count),
                (Self::Sampler(plan), Op::Samplers(count)) => plan.samplers(count),
                _ => panic!("{op:?} is not a run of this heap"),
            }
        }

        fn finish(&self) -> Result<DescriptorHeapLayout, HeapLayoutError> {
            match self {
                Self::Resource(plan) => plan.clone().finish(),
                Self::Sampler(plan) => plan.clone().finish(),
            }
        }
    }

    fn at_least_1(alignment: u64) -> u128 {
        u128::from(alignment.max(1))
    }

    fn least_common_multiple(a: u128, b: u128) -> u128 {
        let (mut x, mut y) = (a, b);
        while y != 0 {
            (x, y) = (y, x % y);
        }
        a * b / x
    }

    fn round_up(value: u128, multiple: u128) -> u128 {
        value.div_ceil(multiple) * multiple
    }

    struct Model {
        properties: Properties,
        heap: Heap,
        end: u128,
    }

    impl Model {
        fn new(properties: Properties, heap: Heap) -> Self {
            Self {
                properties,
                heap,
                end: 0,
            }
        }

        fn run(&mut self, op: Op) -> Result<DescriptorSlots, Vec<HeapLayoutError>> {
            let p = self.properties;
            match op {
                Op::Buffers(count) => self.typed(count, p.buffer_size, p.buffer_alignment),
                Op::Images(count) => self.typed(count, p.image_size, p.image_alignment),
                Op::Samplers(count) => self.typed(count, p.sampler_size, p.sampler_alignment),
                Op::Uniform(count) => self.uniform(count),
            }
        }

        fn typed(
            &mut self,
            count: u32,
            size: u64,
            alignment: u64,
        ) -> Result<DescriptorSlots, Vec<HeapLayoutError>> {
            if size == 0 {
                return Err(vec![HeapLayoutError::ZeroDescriptorSize]);
            }
            let mut errors = Vec::new();
            if !u128::from(size).is_multiple_of(at_least_1(alignment)) {
                errors.push(HeapLayoutError::UnalignedDescriptorSize { size, alignment });
            }
            self.place(count, u128::from(size), size, errors)
        }

        fn uniform(&mut self, count: u32) -> Result<DescriptorSlots, Vec<HeapLayoutError>> {
            let p = self.properties;
            let mut errors = Vec::new();
            if p.buffer_size == 0 || p.image_size == 0 {
                errors.push(HeapLayoutError::ZeroDescriptorSize);
            }
            let both = least_common_multiple(
                at_least_1(p.buffer_alignment),
                at_least_1(p.image_alignment),
            );
            let size = p.buffer_size.max(p.image_size);
            let stride = round_up(u128::from(size), both);
            if both > U64_MAX || stride > U64_MAX {
                errors.push(HeapLayoutError::Overflow);
                return Err(errors);
            }
            if stride == 0 {
                return Err(errors);
            }
            self.place(count, stride, size, errors)
        }

        fn place(
            &mut self,
            count: u32,
            stride: u128,
            descriptor_size: u64,
            mut errors: Vec<HeapLayoutError>,
        ) -> Result<DescriptorSlots, Vec<HeapLayoutError>> {
            let first_index = round_up(self.end, stride) / stride;
            let every_index_fits = first_index <= U32_MAX
                && (count == 0 || first_index + u128::from(count) - 1 <= U32_MAX);
            let end = (first_index + u128::from(count)) * stride;
            if !every_index_fits || end > U64_MAX {
                errors.push(HeapLayoutError::Overflow);
            }
            if !errors.is_empty() {
                return Err(errors);
            }
            self.end = end;
            Ok(DescriptorSlots {
                first_index: u32::try_from(first_index).unwrap(),
                count,
                stride: u64::try_from(stride).unwrap(),
                descriptor_size,
            })
        }

        fn finish(&self) -> Result<DescriptorHeapLayout, HeapLayoutError> {
            let p = self.properties;
            let (reserved_alignment, reserved_size, max, heap_alignment) = match self.heap {
                Heap::Resource => (
                    least_common_multiple(
                        at_least_1(p.buffer_alignment),
                        at_least_1(p.image_alignment),
                    ),
                    p.min_resource_reserved,
                    p.max_resource_heap_size,
                    p.resource_heap_alignment,
                ),
                Heap::Sampler => (
                    at_least_1(p.sampler_alignment),
                    p.min_sampler_reserved,
                    p.max_sampler_heap_size,
                    p.sampler_heap_alignment,
                ),
                Heap::SamplerWithEmbedded => (
                    at_least_1(p.sampler_alignment),
                    p.min_sampler_reserved
                        .max(p.min_sampler_reserved_with_embedded),
                    p.max_sampler_heap_size,
                    p.sampler_heap_alignment,
                ),
            };
            if reserved_alignment > U64_MAX {
                return Err(HeapLayoutError::Overflow);
            }
            let reserved_offset = round_up(self.end, reserved_alignment);
            let size = reserved_offset + u128::from(reserved_size);
            if size > U64_MAX {
                return Err(HeapLayoutError::Overflow);
            }
            let size = u64::try_from(size).unwrap();
            if size > max {
                return Err(HeapLayoutError::TooLarge { size, max });
            }
            Ok(DescriptorHeapLayout {
                size,
                alignment: heap_alignment.max(1),
                reserved_offset: u64::try_from(reserved_offset).unwrap(),
                reserved_size,
            })
        }
    }

    #[derive(Debug, Clone, Copy)]
    enum Address {
        Zero,
        Multiple(u64),
        PastAMultiple(u64, u64),
        HighestThatFits(bool),
        NearTheTop(u64),
        Any(u64),
    }

    impl Address {
        fn resolve(self, layout: &DescriptorHeapLayout) -> u64 {
            let alignment = at_least_1(layout.alignment);
            let address = match self {
                Self::Zero => 0,
                Self::Multiple(k) => alignment * u128::from(k),
                Self::PastAMultiple(k, past) => alignment * u128::from(k) + u128::from(past),
                Self::HighestThatFits(past) => {
                    let highest = ADDRESS_SPACE - u128::from(layout.size);
                    highest / alignment * alignment + if past { alignment } else { 0 }
                }
                Self::NearTheTop(below) => U64_MAX - u128::from(below),
                Self::Any(address) => u128::from(address),
            };
            u64::try_from(address).unwrap_or(u64::MAX)
        }
    }

    fn addresses() -> impl Strategy<Value = Address> {
        prop_oneof![
            1 => Just(Address::Zero),
            4 => (1u64..=1024).prop_map(Address::Multiple),
            2 => (0u64..=1024, 1u64..=3).prop_map(|(k, past)| Address::PastAMultiple(k, past)),
            2 => any::<bool>().prop_map(Address::HighestThatFits),
            1 => (0u64..=3).prop_map(Address::NearTheTop),
            1 => any::<u64>().prop_map(Address::Any),
        ]
    }

    #[derive(Debug, Clone)]
    struct Scenario {
        properties: Properties,
        heap: Heap,
        ops: Vec<Op>,
        address: Address,
        valid_multiple: u64,
    }

    fn pick_within(bound: u64, unit: u64, (choice, random): (u8, u64)) -> u64 {
        match choice % 4 {
            0 => 0,
            1 => bound,
            2 => random % (bound + 1),
            _ => random % (bound / unit + 1) * unit,
        }
    }

    fn pick_at_least(least: u64, (choice, random): (u8, u64)) -> u64 {
        match choice % 4 {
            0 => least,
            1 => least + random % 4096,
            2 => least * 2,
            _ => u64::MAX,
        }
    }

    fn conforming_properties() -> impl Strategy<Value = Properties> {
        (
            (0u32..=7, 0u32..=7),
            (0u32..=6, 0u32..=6),
            (0u32..=5, 0u32..=5),
            (0u32..=16, 0u32..=16),
            prop::array::uniform3((any::<u8>(), any::<u64>())),
            prop::array::uniform2((any::<u8>(), any::<u64>())),
        )
            .prop_map(|(buffer, image, sampler, heap_alignments, reserved, max)| {
                let size_and_alignment =
                    |(size, alignment): (u32, u32)| (1u64 << size, 1u64 << alignment.min(size));
                let (buffer_size, buffer_alignment) = size_and_alignment(buffer);
                let (image_size, image_alignment) = size_and_alignment(image);
                let (sampler_size, sampler_alignment) = size_and_alignment(sampler);
                let largest = buffer_size.max(image_size);
                let min_resource_reserved = pick_within((1 << 15) * largest, largest, reserved[0]);
                let min_sampler_reserved =
                    pick_within(96 * sampler_size, sampler_size, reserved[1]);
                let min_sampler_reserved_with_embedded =
                    pick_within(2048 * sampler_size, sampler_size, reserved[2]);
                Properties {
                    buffer_size,
                    buffer_alignment,
                    image_size,
                    image_alignment,
                    sampler_size,
                    sampler_alignment,
                    resource_heap_alignment: 1 << heap_alignments.0,
                    sampler_heap_alignment: 1 << heap_alignments.1,
                    max_resource_heap_size: pick_at_least(
                        ((1 << 20) - (1 << 15)) * largest + min_resource_reserved,
                        max[0],
                    ),
                    max_sampler_heap_size: pick_at_least(
                        (4000 * sampler_size + min_sampler_reserved)
                            .max(2048 * sampler_size + min_sampler_reserved_with_embedded),
                        max[1],
                    ),
                    min_resource_reserved,
                    min_sampler_reserved,
                    min_sampler_reserved_with_embedded,
                }
            })
    }

    fn extreme_values() -> impl Strategy<Value = u64> {
        prop_oneof![
            2 => Just(0u64),
            3 => prop::sample::select(vec![
                1u64, 2, 3, 5, 6, 8, 12, 16, 24, 32, 48, 64, 72, 96, 128, 256,
            ]),
            2 => (0u32..64).prop_map(|shift| 1u64 << shift),
            2 => prop::sample::select(vec![
                1u64 << 32,
                (1 << 32) + 16,
                1 << 40,
                1 << 62,
                1 << 63,
                (1 << 63) + 1,
                u64::MAX / 3,
                u64::MAX - 1,
                u64::MAX,
            ]),
            1 => any::<u64>(),
        ]
    }

    fn properties() -> impl Strategy<Value = Properties> {
        let replaced = prop_oneof![
            4 => Just(0u16),
            3 => (0u32..13, 0u32..13).prop_map(|(a, b)| (1u16 << a) | (1u16 << b)),
            2 => Just(0x1fff_u16),
        ];
        (
            conforming_properties(),
            prop::array::uniform13(extreme_values()),
            replaced,
        )
            .prop_map(|(mut properties, extreme, replaced)| {
                for (index, (field, value)) in
                    properties.fields_mut().into_iter().zip(extreme).enumerate()
                {
                    if replaced & (1 << index) != 0 {
                        *field = value;
                    }
                }
                properties
            })
    }

    fn counts() -> impl Strategy<Value = u32> {
        prop_oneof![
            4 => Just(0u32),
            3 => Just(1u32),
            4 => 2u32..=16,
            2 => 17u32..=5000,
            1 => (1u32 << 16)..=(1u32 << 20),
            2 => (0u32..=3).prop_map(|below| u32::MAX - below),
            1 => (0u32..=2).prop_map(|past| (1u32 << 31) - 1 + past),
            1 => any::<u32>(),
        ]
    }

    fn scenarios(heaps: Vec<Heap>) -> impl Strategy<Value = Scenario> {
        (
            properties(),
            prop::sample::select(heaps),
            prop::collection::vec((0u8..3, counts()), 0..=8),
            addresses(),
            1u64..=1024,
        )
            .prop_map(|(properties, heap, runs, address, valid_multiple)| {
                let ops = runs
                    .into_iter()
                    .map(|(kind, count)| match (heap, kind) {
                        (Heap::Resource, 0) => Op::Buffers(count),
                        (Heap::Resource, 1) => Op::Images(count),
                        (Heap::Resource, _) => Op::Uniform(count),
                        _ => Op::Samplers(count),
                    })
                    .collect();
                Scenario {
                    properties,
                    heap,
                    ops,
                    address,
                    valid_multiple,
                }
            })
    }

    fn every_heap() -> Vec<Heap> {
        vec![Heap::Resource, Heap::Sampler, Heap::SamplerWithEmbedded]
    }

    #[derive(Debug, Default)]
    struct Seen {
        runs_placed: u32,
        empty_runs_that_moved_the_end: u32,
        uniform_strides_rounded_up: u32,
        zero_descriptor_size: u32,
        unaligned_descriptor_size: u32,
        run_overflow: u32,
        runs_after_a_rejected_run: u32,
        layouts: u32,
        too_large: u32,
        finish_overflow: u32,
        conforming_layouts_checked: u32,
        conforming_runs_checked: u32,
        bind_ok: u32,
        bind_reserved_range_outside_heap: u32,
        bind_misaligned: u32,
        bind_null_address: u32,
        bind_address_range_overflow: u32,
        slot_outside_the_run: u32,
        index_past_u32: u32,
        offset_past_u64: u32,
        range_past_usize: u32,
        bytes_past_the_heap: u32,
        bytes_in_the_heap: u32,
        slices_with_a_stride_of_0: u32,
        slices_of_an_empty_run: u32,
        slices_in_the_heap: u32,
        slices_cut_to_the_stride: u32,
        slices_past_the_heap: u32,
        slices_ending_with_the_last_descriptor: u32,
    }

    fn check_plan(scenario: &Scenario, seen: &mut Seen) -> Result<(), String> {
        let mut plan = Plan::new(&scenario.properties, scenario.heap);
        let mut model = Model::new(scenario.properties, scenario.heap);
        let mut rejected = 0;
        for (step, &op) in scenario.ops.iter().enumerate() {
            let end_before = model.end;
            let expected = model.run(op);
            let actual = plan.apply(op);
            if rejected > 0 {
                seen.runs_after_a_rejected_run += 1;
            }
            match (&expected, &actual) {
                (Ok(expected), Ok(actual)) if expected == actual => {
                    seen.runs_placed += 1;
                    if actual.count == 0 && model.end != end_before {
                        seen.empty_runs_that_moved_the_end += 1;
                    }
                    if matches!(op, Op::Uniform(_)) && actual.stride > actual.descriptor_size {
                        seen.uniform_strides_rounded_up += 1;
                    }
                }
                (Err(errors), Err(actual)) if errors.first() == Some(actual) => {
                    rejected += 1;
                    match actual {
                        HeapLayoutError::ZeroDescriptorSize => seen.zero_descriptor_size += 1,
                        HeapLayoutError::UnalignedDescriptorSize { .. } => {
                            seen.unaligned_descriptor_size += 1;
                        }
                        _ => seen.run_overflow += 1,
                    }
                }
                _ => {
                    return Err(format!(
                        "step {step}, {op:?}, after {rejected} rejected runs that must leave \
                         the plan unchanged: the plan gave {actual:?}, the documentation gives \
                         {expected:?}"
                    ));
                }
            }
            let layout = plan.finish();
            let expected_layout = model.finish();
            if layout != expected_layout {
                return Err(format!(
                    "finish after step {step}, {op:?}, after {rejected} rejected runs: the plan \
                     gave {layout:?}, the documentation gives {expected_layout:?}"
                ));
            }
        }
        match plan.finish() {
            Ok(_) => seen.layouts += 1,
            Err(HeapLayoutError::TooLarge { .. }) => seen.too_large += 1,
            Err(_) => seen.finish_overflow += 1,
        }
        Ok(())
    }

    fn meets_the_required_limits(p: &Properties) -> bool {
        let power_of_two_at_most = |value: u64, most: u64| value.is_power_of_two() && value <= most;
        let largest = u128::from(p.buffer_size.max(p.image_size));
        let sampler_size = u128::from(p.sampler_size);
        power_of_two_at_most(p.sampler_size, 32)
            && power_of_two_at_most(p.image_size, 64)
            && power_of_two_at_most(p.buffer_size, 128)
            && power_of_two_at_most(p.sampler_alignment, p.sampler_size.min(32))
            && power_of_two_at_most(p.image_alignment, p.image_size.min(64))
            && power_of_two_at_most(p.buffer_alignment, p.buffer_size.min(128))
            && power_of_two_at_most(p.sampler_heap_alignment, 65536)
            && power_of_two_at_most(p.resource_heap_alignment, 65536)
            && u128::from(p.min_sampler_reserved) <= 96 * sampler_size
            && u128::from(p.min_sampler_reserved_with_embedded) <= 2048 * sampler_size
            && u128::from(p.min_resource_reserved) <= (1 << 15) * largest
            && u128::from(p.max_sampler_heap_size)
                >= (4000 * sampler_size + u128::from(p.min_sampler_reserved))
                    .max(2048 * sampler_size + u128::from(p.min_sampler_reserved_with_embedded))
            && u128::from(p.max_resource_heap_size)
                >= ((1 << 20) - (1 << 15)) * largest + u128::from(p.min_resource_reserved)
    }

    struct BindRules {
        reserved_range_in_the_heap: &'static str,
        size_at_most_the_maximum: &'static str,
        address_alignment: &'static str,
        reserved_offset_alignments: Vec<(u64, &'static str)>,
        min_reserved: Vec<(u64, &'static str)>,
        max_size: u64,
        heap_alignment: u64,
    }

    fn bind_rules(p: &Properties, heap: Heap) -> BindRules {
        match heap {
            Heap::Resource => BindRules {
                reserved_range_in_the_heap: "VUID-vkCmdBindResourceHeapEXT-pBindInfo-11232",
                size_at_most_the_maximum: "VUID-vkCmdBindResourceHeapEXT-pBindInfo-11234",
                address_alignment: "VUID-vkCmdBindResourceHeapEXT-pBindInfo-11235",
                reserved_offset_alignments: vec![
                    (
                        p.buffer_alignment,
                        "VUID-vkCmdBindResourceHeapEXT-pBindInfo-11435",
                    ),
                    (
                        p.image_alignment,
                        "VUID-vkCmdBindResourceHeapEXT-pBindInfo-11436",
                    ),
                ],
                min_reserved: vec![(
                    p.min_resource_reserved,
                    "VUID-vkCmdBindResourceHeapEXT-pBindInfo-11233",
                )],
                max_size: p.max_resource_heap_size,
                heap_alignment: p.resource_heap_alignment,
            },
            Heap::Sampler | Heap::SamplerWithEmbedded => BindRules {
                reserved_range_in_the_heap: "VUID-vkCmdBindSamplerHeapEXT-pBindInfo-11223",
                size_at_most_the_maximum: "VUID-vkCmdBindSamplerHeapEXT-pBindInfo-11225",
                address_alignment: "VUID-vkCmdBindSamplerHeapEXT-pBindInfo-11226",
                reserved_offset_alignments: vec![(
                    p.sampler_alignment,
                    "VUID-vkCmdBindSamplerHeapEXT-pBindInfo-11434",
                )],
                min_reserved: if heap == Heap::SamplerWithEmbedded {
                    vec![
                        (
                            p.min_sampler_reserved,
                            "VUID-vkCmdBindSamplerHeapEXT-pBindInfo-11224",
                        ),
                        (
                            p.min_sampler_reserved_with_embedded,
                            "VUID-vkCmdDraw-pBindInfo-11375",
                        ),
                    ]
                } else {
                    vec![(
                        p.min_sampler_reserved,
                        "VUID-vkCmdBindSamplerHeapEXT-pBindInfo-11224",
                    )]
                },
                max_size: p.max_sampler_heap_size,
                heap_alignment: p.sampler_heap_alignment,
            },
        }
    }

    fn check_bind_against_the_specification(
        rules: &BindRules,
        bind: &vk::BindHeapInfoEXT<'_>,
    ) -> Result<(), String> {
        let address = bind.heap_range.address;
        let size = bind.heap_range.size;
        let reserved_offset = bind.reserved_range_offset;
        let reserved_size = bind.reserved_range_size;
        let mut problems = Vec::new();
        if u128::from(reserved_offset) + u128::from(reserved_size) > u128::from(size) {
            problems.push(format!(
                "{}: the reserved range {reserved_offset} + {reserved_size} is past the heap \
                 size {size}",
                rules.reserved_range_in_the_heap
            ));
        }
        for &(min, vuid) in &rules.min_reserved {
            if reserved_size < min {
                problems.push(format!(
                    "{vuid}: the reserved range of {reserved_size} bytes is below {min}"
                ));
            }
        }
        if size > rules.max_size {
            problems.push(format!(
                "{}: the heap of {size} bytes is larger than {}",
                rules.size_at_most_the_maximum, rules.max_size
            ));
        }
        if !address.is_multiple_of(rules.heap_alignment) {
            problems.push(format!(
                "{}: address {address:#x} is not a multiple of {}",
                rules.address_alignment, rules.heap_alignment
            ));
        }
        for &(alignment, vuid) in &rules.reserved_offset_alignments {
            if !reserved_offset.is_multiple_of(alignment) {
                problems.push(format!(
                    "{vuid}: the reserved offset {reserved_offset} is not a multiple of \
                     {alignment}"
                ));
            }
        }
        if size != 0 && address == 0 {
            problems.push(format!(
                "VUID-VkDeviceAddressRangeKHR-size-11411: a heap of {size} bytes at address 0"
            ));
        }
        if u128::from(address) + u128::from(size) > ADDRESS_SPACE {
            problems.push(format!(
                "VUID-VkDeviceAddressRangeKHR-address-11365: a heap of {size} bytes at \
                 {address:#x} reaches past the address space"
            ));
        }
        if problems.is_empty() {
            Ok(())
        } else {
            Err(problems.join("; "))
        }
    }

    type Requirements = Vec<(u64, &'static str)>;

    fn run_rules(p: &Properties, op: Op) -> (Requirements, Requirements) {
        let image = (
            p.image_alignment,
            "VUID-VkDescriptorSetAndBindingMappingEXT-source-11251",
        );
        let buffer = (
            p.buffer_alignment,
            "VUID-VkDescriptorSetAndBindingMappingEXT-source-11252",
        );
        let sampler = (
            p.sampler_alignment,
            "VUID-VkDescriptorSetAndBindingMappingEXT-source-11253",
        );
        let image_write = (
            p.image_size,
            "VUID-vkWriteResourceDescriptorsEXT-size-11207",
        );
        let buffer_write = (
            p.buffer_size,
            "VUID-vkWriteResourceDescriptorsEXT-size-11207",
        );
        let sampler_write = (
            p.sampler_size,
            "VUID-vkWriteSamplerDescriptorsEXT-size-11203",
        );
        match op {
            Op::Buffers(_) => (vec![buffer], vec![buffer_write]),
            Op::Images(_) => (vec![image], vec![image_write]),
            Op::Uniform(_) => (vec![image, buffer], vec![image_write, buffer_write]),
            Op::Samplers(_) => (vec![sampler], vec![sampler_write]),
        }
    }

    fn check_the_specification(scenario: &Scenario, seen: &mut Seen) -> Result<(), String> {
        let p = &scenario.properties;
        if !meets_the_required_limits(p) {
            return Ok(());
        }
        let mut plan = Plan::new(p, scenario.heap);
        let runs: Vec<(Op, DescriptorSlots)> = scenario
            .ops
            .iter()
            .filter_map(|&op| plan.apply(op).ok().map(|slots| (op, slots)))
            .collect();
        let Ok(layout) = plan.finish() else {
            return Ok(());
        };
        seen.conforming_layouts_checked += 1;
        let rules = bind_rules(p, scenario.heap);

        let valid_address = rules.heap_alignment * scenario.valid_multiple;
        if u128::from(valid_address) + u128::from(layout.size) <= ADDRESS_SPACE {
            let bind = layout.bind_info(valid_address).map_err(|error| {
                format!(
                    "{layout:?} cannot be bound at {valid_address:#x}, a multiple of the heap \
                     alignment {}: {error:?}",
                    rules.heap_alignment
                )
            })?;
            if (
                bind.heap_range.address,
                bind.heap_range.size,
                bind.reserved_range_offset,
                bind.reserved_range_size,
            ) != (
                valid_address,
                layout.size,
                layout.reserved_offset,
                layout.reserved_size,
            ) {
                return Err(format!(
                    "the VkBindHeapInfoEXT for {layout:?} at {valid_address:#x} does not name the \
                     layout"
                ));
            }
            check_bind_against_the_specification(&rules, &bind)?;
        }
        let address = scenario.address.resolve(&layout);
        if let Ok(bind) = layout.bind_info(address) {
            check_bind_against_the_specification(&rules, &bind)?;
        }

        let reserved = (
            u128::from(layout.reserved_offset),
            u128::from(layout.reserved_offset) + u128::from(layout.reserved_size),
        );
        let mut placed: Vec<(u128, u128, Op)> = Vec::new();
        for &(op, slots) in &runs {
            let (alignments, writes) = run_rules(p, op);
            for &(alignment, vuid) in &alignments {
                if !slots.stride.is_multiple_of(alignment) {
                    return Err(format!(
                        "{vuid}: {op:?} gave {slots:?}, whose stride as heapArrayStride is not \
                         a multiple of {alignment}"
                    ));
                }
            }
            if slots.count == 0 {
                continue;
            }
            seen.conforming_runs_checked += 1;
            if slots.descriptor_size > slots.stride {
                return Err(format!(
                    "{op:?} gave {slots:?}, whose descriptors overlap the next slot"
                ));
            }
            for slot in [0, slots.count - 1] {
                let offset = slots.offset(slot).ok_or_else(|| {
                    format!("{op:?} gave {slots:?}, which has no offset for slot {slot}")
                })?;
                for &(alignment, vuid) in &alignments {
                    if !offset.is_multiple_of(alignment) {
                        return Err(format!(
                            "{vuid}: {op:?} gave {slots:?}, whose slot {slot} at heapOffset \
                             {offset} is not a multiple of {alignment}"
                        ));
                    }
                }
            }
            for &(size, vuid) in &writes {
                if slots.descriptor_size < size {
                    return Err(format!(
                        "{vuid}: {op:?} gave {slots:?}, whose slots are smaller than a \
                         descriptor of {size} bytes"
                    ));
                }
            }
            let start = u128::from(slots.offset(0).unwrap_or(u64::MAX));
            let end = u128::from(slots.offset(slots.count - 1).unwrap_or(u64::MAX))
                + u128::from(slots.descriptor_size);
            if end > u128::from(layout.size) {
                return Err(format!(
                    "{op:?} gave {slots:?}, which ends at {end}, past the heap range of {} bytes",
                    layout.size
                ));
            }
            if start < reserved.1 && reserved.0 < end {
                return Err(format!(
                    "descriptorheaps-reservedranges: {op:?} gave {slots:?}, which reaches into \
                     the reserved range {reserved:?} of {layout:?}"
                ));
            }
            if let Some(&(other_start, other_end, other)) = placed
                .iter()
                .find(|&&(other_start, other_end, _)| start < other_end && other_start < end)
            {
                return Err(format!(
                    "{op:?} gave {slots:?} over bytes {start}..{end}, which overlap the bytes \
                     {other_start}..{other_end} of {other:?}"
                ));
            }
            placed.push((start, end, op));
        }
        Ok(())
    }

    fn documented_bind_info(
        layout: &DescriptorHeapLayout,
        address: u64,
    ) -> Result<(u64, u64, u64, u64), HeapLayoutError> {
        if u128::from(layout.reserved_offset) + u128::from(layout.reserved_size)
            > u128::from(layout.size)
        {
            return Err(HeapLayoutError::ReservedRangeOutsideHeap {
                reserved_offset: layout.reserved_offset,
                reserved_size: layout.reserved_size,
                heap_size: layout.size,
            });
        }
        if !u128::from(address).is_multiple_of(at_least_1(layout.alignment)) {
            return Err(HeapLayoutError::Misaligned {
                address,
                alignment: layout.alignment,
            });
        }
        if address == 0 {
            return Err(HeapLayoutError::NullAddress);
        }
        if layout.size != 0 && u128::from(address) + u128::from(layout.size) - 1 > U64_MAX {
            return Err(HeapLayoutError::AddressRangeOverflow {
                address,
                size: layout.size,
            });
        }
        Ok((
            address,
            layout.size,
            layout.reserved_offset,
            layout.reserved_size,
        ))
    }

    fn check_bind_info(
        layout: &DescriptorHeapLayout,
        address: u64,
        seen: &mut Seen,
    ) -> Result<(), String> {
        let expected = documented_bind_info(layout, address);
        let actual = layout.bind_info(address);
        if let Ok(bind) = &actual
            && (bind.s_type != vk::StructureType::BIND_HEAP_INFO_EXT || !bind.p_next.is_null())
        {
            return Err(format!(
                "{layout:?} at {address:#x}: the VkBindHeapInfoEXT has sType {:?} and a pNext \
                 that is null: {}",
                bind.s_type,
                bind.p_next.is_null()
            ));
        }
        let actual = actual.map(|bind| {
            (
                bind.heap_range.address,
                bind.heap_range.size,
                bind.reserved_range_offset,
                bind.reserved_range_size,
            )
        });
        if actual != expected {
            return Err(format!(
                "{layout:?} at {address:#x}: bind_info gave {actual:?}, the first documented \
                 error or the layout is {expected:?}"
            ));
        }
        match actual {
            Ok(_) => seen.bind_ok += 1,
            Err(HeapLayoutError::ReservedRangeOutsideHeap { .. }) => {
                seen.bind_reserved_range_outside_heap += 1;
            }
            Err(HeapLayoutError::Misaligned { .. }) => seen.bind_misaligned += 1,
            Err(HeapLayoutError::NullAddress) => seen.bind_null_address += 1,
            Err(_) => seen.bind_address_range_overflow += 1,
        }
        Ok(())
    }

    fn heap_sizes() -> impl Strategy<Value = u64> {
        prop_oneof![
            2 => Just(0u64),
            1 => Just(1u64),
            3 => prop::sample::select(vec![2u64, 0x100, 0x1000, 0x2_0000, 1 << 32]),
            2 => prop::sample::select(vec![1u64 << 63, u64::MAX - 1, u64::MAX]),
            1 => any::<u64>(),
        ]
    }

    fn heap_alignments() -> impl Strategy<Value = u64> {
        prop_oneof![
            1 => Just(0u64),
            1 => Just(1u64),
            3 => (1u32..=16).prop_map(|shift| 1u64 << shift),
            1 => prop::sample::select(vec![3u64, 24, 1 << 63, u64::MAX]),
            1 => any::<u64>(),
        ]
    }

    fn layouts() -> impl Strategy<Value = DescriptorHeapLayout> {
        (
            heap_sizes(),
            heap_alignments(),
            (0u8..5, any::<u64>()),
            (0u8..5, any::<u64>()),
        )
            .prop_map(
                |(size, alignment, (size_choice, size_random), (offset_choice, offset_random))| {
                    let reserved_size = match size_choice {
                        0 => 0,
                        1 => size,
                        2 => {
                            u64::try_from(u128::from(size_random) % (u128::from(size) + 1)).unwrap()
                        }
                        3 => size / 2,
                        _ => [1, 0x100, u64::MAX][(size_random % 3) as usize],
                    };
                    let room = size.saturating_sub(reserved_size);
                    let reserved_offset = match offset_choice {
                        0 => room,
                        1 => 0,
                        2 => u64::try_from(u128::from(offset_random) % (u128::from(room) + 1))
                            .unwrap(),
                        3 => room.saturating_add(1),
                        _ => [1, 1 << 63, u64::MAX][(offset_random % 3) as usize],
                    };
                    DescriptorHeapLayout {
                        size,
                        alignment,
                        reserved_offset,
                        reserved_size,
                    }
                },
            )
    }

    fn documented_index(slots: &DescriptorSlots, slot: u32) -> Option<u128> {
        let index = u128::from(slots.first_index) + u128::from(slot);
        (slot < slots.count && index <= U32_MAX).then_some(index)
    }

    fn documented_offset(slots: &DescriptorSlots, slot: u32) -> Option<u128> {
        let offset = documented_index(slots, slot)? * u128::from(slots.stride);
        (offset <= U64_MAX).then_some(offset)
    }

    fn documented_range(slots: &DescriptorSlots, slot: u32) -> Option<(u128, u128)> {
        let start = documented_offset(slots, slot)?;
        let end = start + u128::from(slots.descriptor_size);
        (end <= USIZE_MAX).then_some((start, end))
    }

    fn documented_slices(slots: &DescriptorSlots, heap_length: usize) -> Option<Vec<(u128, u128)>> {
        if slots.stride == 0 {
            return None;
        }
        let length = u128::from(slots.descriptor_size.min(slots.stride));
        let slice = |slot: u32| -> Option<(u128, u128)> {
            let start = documented_offset(slots, slot)?;
            let end = start + length;
            (end <= USIZE_MAX && end <= heap_length as u128).then_some((start, end))
        };
        if let Some(last) = slots.count.checked_sub(1) {
            slice(last)?;
        }
        (0..slots.count).map(slice).collect()
    }

    #[derive(Debug, Clone, Copy)]
    struct SlotCase {
        slots: DescriptorSlots,
        slot: u32,
        heap_length: usize,
    }

    fn slot_cases() -> impl Strategy<Value = SlotCase> {
        let first_index = prop_oneof![
            3 => Just(0u32),
            2 => Just(1u32),
            3 => 2u32..64,
            1 => 64u32..4096,
            2 => (0u32..=3).prop_map(|below| u32::MAX - below),
            1 => any::<u32>(),
        ];
        let count = prop_oneof![
            2 => Just(0u32),
            3 => Just(1u32),
            3 => 2u32..8,
            2 => 8u32..300,
            1 => (0u32..=3).prop_map(|below| u32::MAX - below),
            1 => any::<u32>(),
        ];
        let stride = prop_oneof![
            2 => Just(0u64),
            2 => Just(1u64),
            3 => 2u64..=16,
            3 => prop::sample::select(vec![24u64, 32, 48, 64, 72, 96, 128, 256]),
            2 => (0u32..64).prop_map(|shift| 1u64 << shift),
            1 => (0u64..=3).prop_map(|below| u64::MAX - below),
            1 => (0u64..=2).prop_map(|past| u64::MAX / 2 + past),
            1 => any::<u64>(),
        ];
        (
            first_index,
            count,
            stride,
            (0u8..8, any::<u64>()),
            (0u8..7, any::<u32>()),
            (0u8..6, any::<u64>()),
        )
            .prop_map(
                |(
                    first_index,
                    count,
                    stride,
                    (size_choice, size_random),
                    (slot_choice, slot_random),
                    (heap_choice, heap_random),
                )| {
                    let descriptor_size = match size_choice {
                        0 => 0,
                        1 => 1,
                        2 | 3 => stride,
                        4 => stride.saturating_sub(1),
                        5 => stride.saturating_add(1 + size_random % 64),
                        6 => size_random % stride.max(1),
                        _ => [u64::MAX, 1 << 32, size_random][(size_random % 3) as usize],
                    };
                    let slot = match slot_choice {
                        0 => 0,
                        1 => 1,
                        2 | 3 => count.saturating_sub(1),
                        4 => count,
                        5 => u32::MAX,
                        _ => slot_random,
                    };
                    let slot_end = |slot: u32, size: u64| {
                        (u128::from(first_index) + u128::from(slot)) * u128::from(stride)
                            + u128::from(size)
                    };
                    let last = count.saturating_sub(1);
                    let heap_length = match heap_choice {
                        0 => slot_end(last, descriptor_size.min(stride)),
                        1 => slot_end(last, descriptor_size.min(stride)).saturating_sub(1),
                        2 => slot_end(slot, descriptor_size),
                        3 => slot_end(slot, descriptor_size).saturating_sub(1),
                        4 => slot_end(last, stride),
                        _ => u128::from(heap_random % (HEAP_BYTES as u64 + 1)),
                    }
                    .min(HEAP_BYTES as u128);
                    SlotCase {
                        slots: DescriptorSlots {
                            first_index,
                            count,
                            stride,
                            descriptor_size,
                        },
                        slot,
                        heap_length: usize::try_from(heap_length).unwrap(),
                    }
                },
            )
    }

    fn offsets_in<'a>(
        heap: *const u8,
        slices: impl Iterator<Item = &'a mut [u8]>,
        most: usize,
    ) -> Vec<(u128, u128)> {
        slices
            .take(most)
            .map(|slice| {
                let start = (slice.as_ptr() as usize - heap as usize) as u128;
                (start, start + slice.len() as u128)
            })
            .collect()
    }

    fn check_slots(case: &SlotCase, seen: &mut Seen) -> Result<(), String> {
        let SlotCase {
            slots,
            slot,
            heap_length,
        } = *case;
        let mut buffer = vec![0u8; HEAP_BYTES];
        let heap = &mut buffer[..heap_length];
        let base = heap.as_ptr();

        let index = slots.index(slot).map(u128::from);
        if index != documented_index(&slots, slot) {
            return Err(format!(
                "index({slot}) gave {index:?}, the documentation gives {:?}",
                documented_index(&slots, slot)
            ));
        }
        let offset = slots.offset(slot).map(u128::from);
        if offset != documented_offset(&slots, slot) {
            return Err(format!(
                "offset({slot}) gave {offset:?}, the documentation gives {:?}",
                documented_offset(&slots, slot)
            ));
        }
        let range = slots
            .range(slot)
            .map(|range| (range.start as u128, range.end as u128));
        if range != documented_range(&slots, slot) {
            return Err(format!(
                "range({slot}) gave {range:?}, the documentation gives {:?}",
                documented_range(&slots, slot)
            ));
        }
        let expected_end = ((u128::from(slots.first_index) + u128::from(slots.count))
            * u128::from(slots.stride))
        .min(U64_MAX);
        if u128::from(slots.end()) != expected_end {
            return Err(format!(
                "end() gave {}, the documentation gives {expected_end}",
                slots.end()
            ));
        }
        let expected_bytes =
            documented_range(&slots, slot).filter(|&(_, end)| end <= heap_length as u128);
        let bytes = slots.bytes(heap, slot).map(|bytes| {
            let start = (bytes.as_ptr() as usize - base as usize) as u128;
            (start, start + bytes.len() as u128)
        });
        if bytes != expected_bytes {
            return Err(format!(
                "bytes of a {heap_length}-byte heap, slot {slot}, gave {bytes:?}, the \
                 documentation gives {expected_bytes:?}"
            ));
        }
        let expected_slices = documented_slices(&slots, heap_length);
        let most = expected_slices.as_ref().map_or(0, Vec::len) + 1;
        let slices = slots
            .slices(heap)
            .map(|slices| offsets_in(base, slices, most));
        if slices != expected_slices {
            return Err(format!(
                "slices of a {heap_length}-byte heap gave {slices:?}, the documentation gives \
                 {expected_slices:?}"
            ));
        }

        if slot >= slots.count {
            seen.slot_outside_the_run += 1;
        } else if index.is_none() {
            seen.index_past_u32 += 1;
        } else if offset.is_none() {
            seen.offset_past_u64 += 1;
        } else if range.is_none() {
            seen.range_past_usize += 1;
        } else if bytes.is_none() {
            seen.bytes_past_the_heap += 1;
        } else {
            seen.bytes_in_the_heap += 1;
        }
        match &slices {
            None if slots.stride == 0 => seen.slices_with_a_stride_of_0 += 1,
            None => seen.slices_past_the_heap += 1,
            Some(slices) if slices.is_empty() => seen.slices_of_an_empty_run += 1,
            Some(slices) => {
                seen.slices_in_the_heap += 1;
                if slots.descriptor_size > slots.stride {
                    seen.slices_cut_to_the_stride += 1;
                }
                if slices
                    .last()
                    .is_some_and(|&(_, end)| end == heap_length as u128)
                {
                    seen.slices_ending_with_the_last_descriptor += 1;
                }
            }
        }
        Ok(())
    }

    fn check_slices_against_bytes(case: &SlotCase) -> Result<(), String> {
        let stride = case.slots.stride.max(1);
        let slots = DescriptorSlots {
            stride,
            descriptor_size: case.slots.descriptor_size.min(stride),
            ..case.slots
        };
        let mut buffer = vec![0u8; HEAP_BYTES];
        let heap = &mut buffer[..case.heap_length];
        let base = heap.as_ptr();
        let span = |bytes: &mut [u8]| {
            let start = (bytes.as_ptr() as usize - base as usize) as u128;
            (start, start + bytes.len() as u128)
        };
        let every_bytes: Option<Vec<(u128, u128)>> = if slots.count as usize <= HEAP_BYTES + 1 {
            (0..slots.count)
                .map(|slot| slots.bytes(heap, slot).map(span))
                .collect()
        } else if slots.bytes(heap, slots.count - 1).is_none() {
            None
        } else {
            return Err(format!(
                "{slots:?}: the last of {} slots fits in a heap of {} bytes",
                slots.count, case.heap_length
            ));
        };
        let most = every_bytes.as_ref().map_or(0, Vec::len) + 1;
        let slices = slots
            .slices(heap)
            .map(|slices| offsets_in(base, slices, most));
        if slices != every_bytes {
            return Err(format!(
                "{slots:?} in a heap of {} bytes: slices gave {slices:?}, but bytes of every \
                 slot gives {every_bytes:?}",
                case.heap_length
            ));
        }
        Ok(())
    }

    fn cases(default: u32) -> ProptestConfig {
        let cases = std::env::var("PROPTEST_CASES")
            .ok()
            .and_then(|cases| cases.parse().ok())
            .unwrap_or(default);
        ProptestConfig {
            cases,
            failure_persistence: None,
            ..ProptestConfig::default()
        }
    }

    proptest! {
        #![proptest_config(cases(2000))]

        #[test]
        fn plans_give_the_runs_errors_and_layouts_the_documentation_describes(
            scenario in scenarios(every_heap())
        ) {
            check_plan(&scenario, &mut Seen::default()).map_err(TestCaseError::fail)?;
        }

        #[test]
        fn conforming_resource_heaps_meet_vuid_11232_11233_11234_11235_11251_11252_11365_11411_11435_11436_and_11207(
            scenario in scenarios(vec![Heap::Resource])
        ) {
            check_the_specification(&scenario, &mut Seen::default())
                .map_err(TestCaseError::fail)?;
        }

        #[test]
        fn conforming_sampler_heaps_meet_vuid_11223_11224_11225_11226_11253_11365_11375_11411_11434_and_11203(
            scenario in scenarios(vec![Heap::Sampler, Heap::SamplerWithEmbedded])
        ) {
            check_the_specification(&scenario, &mut Seen::default())
                .map_err(TestCaseError::fail)?;
        }
    }

    proptest! {
        #![proptest_config(cases(5000))]

        #[test]
        fn bind_info_gives_the_first_documented_error_or_the_layout(
            layout in layouts(),
            address in addresses(),
        ) {
            check_bind_info(&layout, address.resolve(&layout), &mut Seen::default())
                .map_err(TestCaseError::fail)?;
        }

        #[test]
        fn slot_accessors_follow_the_documented_arithmetic_without_panicking(
            case in slot_cases()
        ) {
            check_slots(&case, &mut Seen::default()).map_err(TestCaseError::fail)?;
        }

        #[test]
        fn slices_are_the_bytes_of_every_slot_when_descriptors_fit_their_stride(
            case in slot_cases()
        ) {
            check_slices_against_bytes(&case).map_err(TestCaseError::fail)?;
        }
    }

    #[test]
    fn the_generated_inputs_reach_every_error_and_success() {
        let mut runner = TestRunner::new_with_rng(
            ProptestConfig::default(),
            TestRng::deterministic_rng(RngAlgorithm::ChaCha),
        );
        let mut seen = Seen::default();
        let cases = 2000;

        let conforming = conforming_properties();
        for _ in 0..cases {
            let properties = conforming.new_tree(&mut runner).unwrap().current();
            assert!(
                meets_the_required_limits(&properties),
                "the conforming generator made {properties:?}"
            );
        }

        let plans = scenarios(every_heap());
        let mut conforming_scenarios = 0;
        for _ in 0..cases {
            let scenario = plans.new_tree(&mut runner).unwrap().current();
            if meets_the_required_limits(&scenario.properties) {
                conforming_scenarios += 1;
            }
            check_plan(&scenario, &mut seen).unwrap();
            check_the_specification(&scenario, &mut seen).unwrap();
        }
        let bind = (layouts(), addresses());
        for _ in 0..cases {
            let (layout, address) = bind.new_tree(&mut runner).unwrap().current();
            check_bind_info(&layout, address.resolve(&layout), &mut seen).unwrap();
        }
        let slots = slot_cases();
        for _ in 0..cases {
            let case = slots.new_tree(&mut runner).unwrap().current();
            check_slots(&case, &mut seen).unwrap();
        }

        let runs = seen.runs_placed
            + seen.zero_descriptor_size
            + seen.unaligned_descriptor_size
            + seen.run_overflow;
        let finishes = seen.layouts + seen.too_large + seen.finish_overflow;
        assert!(
            conforming_scenarios * 4 > cases,
            "{conforming_scenarios} of {cases} scenarios have conforming properties"
        );
        assert!(seen.runs_placed * 3 > runs, "{seen:?}");
        assert!(seen.run_overflow * 20 > runs, "{seen:?}");
        assert!(seen.unaligned_descriptor_size * 20 > runs, "{seen:?}");
        assert!(seen.zero_descriptor_size * 50 > runs, "{seen:?}");
        assert!(seen.runs_after_a_rejected_run * 10 > runs, "{seen:?}");
        assert!(seen.empty_runs_that_moved_the_end > 20, "{seen:?}");
        assert!(seen.uniform_strides_rounded_up > 50, "{seen:?}");
        assert!(seen.layouts * 3 > finishes, "{seen:?}");
        assert!(seen.too_large * 10 > finishes, "{seen:?}");
        assert!(seen.finish_overflow * 40 > finishes, "{seen:?}");
        assert!(seen.conforming_layouts_checked * 8 > cases, "{seen:?}");
        assert!(seen.conforming_runs_checked * 4 > cases, "{seen:?}");

        let binds = seen.bind_ok
            + seen.bind_reserved_range_outside_heap
            + seen.bind_misaligned
            + seen.bind_null_address
            + seen.bind_address_range_overflow;
        for (name, count) in [
            ("Ok", seen.bind_ok),
            (
                "ReservedRangeOutsideHeap",
                seen.bind_reserved_range_outside_heap,
            ),
            ("Misaligned", seen.bind_misaligned),
            ("NullAddress", seen.bind_null_address),
            ("AddressRangeOverflow", seen.bind_address_range_overflow),
        ] {
            assert!(
                count * 25 > binds,
                "bind_info gave {name} {count} times: {seen:?}"
            );
        }

        for (name, count) in [
            ("a slot outside the run", seen.slot_outside_the_run),
            ("an index past u32", seen.index_past_u32),
            ("an offset past u64", seen.offset_past_u64),
            ("bytes past the heap", seen.bytes_past_the_heap),
            ("bytes in the heap", seen.bytes_in_the_heap),
            ("slices with a stride of 0", seen.slices_with_a_stride_of_0),
            ("slices of an empty run", seen.slices_of_an_empty_run),
            ("slices in the heap", seen.slices_in_the_heap),
            ("slices cut to the stride", seen.slices_cut_to_the_stride),
            ("slices past the heap", seen.slices_past_the_heap),
            (
                "slices ending with the last descriptor",
                seen.slices_ending_with_the_last_descriptor,
            ),
        ] {
            assert!(count * 100 > cases, "{name}: {count} of {cases}: {seen:?}");
        }
        assert!(seen.range_past_usize * 200 > cases, "{seen:?}");
    }

    #[test]
    fn properties_that_were_not_queried_have_zero_descriptor_sizes() {
        let unqueried = vk::PhysicalDeviceDescriptorHeapPropertiesEXT::default();
        let mut plan = ResourceHeapPlan::new(&unqueried);
        assert_eq!(plan.buffers(1), Err(HeapLayoutError::ZeroDescriptorSize));
        assert_eq!(plan.images(0), Err(HeapLayoutError::ZeroDescriptorSize));
        assert_eq!(plan.uniform(1), Err(HeapLayoutError::ZeroDescriptorSize));
        assert_eq!(
            SamplerHeapPlan::new(&unqueried).samplers(1),
            Err(HeapLayoutError::ZeroDescriptorSize)
        );
    }

    #[test]
    fn a_run_fits_exactly_when_every_index_of_it_fits_in_u32() {
        let near_the_limit = [
            0,
            1,
            2,
            1 << 31,
            (1 << 31) + 1,
            u32::MAX - 2,
            u32::MAX - 1,
            u32::MAX,
        ];
        let properties = vk::PhysicalDeviceDescriptorHeapPropertiesEXT::default()
            .max_resource_heap_size(1 << 16)
            .image_descriptor_size(72)
            .image_descriptor_alignment(24);
        let stride = properties.image_descriptor_size;
        let mut fitting = 0;
        let mut overflowing = 0;
        for first in near_the_limit {
            for count in near_the_limit {
                let context = format!("first index {first}, count {count}");
                let mut plan = ResourceHeapPlan::new(&properties);
                assert_eq!(
                    plan.images(first).map(|slots| slots.end()),
                    Ok(u64::from(first) * stride),
                    "{context}"
                );
                let end = u64::from(first) + u64::from(count);
                let every_index_fits = count == 0 || end - 1 <= u64::from(u32::MAX);
                match plan.images(count) {
                    Ok(slots) => {
                        fitting += 1;
                        assert!(every_index_fits, "{context}");
                        assert_eq!(slots.first_index, first, "{context}");
                        assert_eq!(slots.count, count, "{context}");
                        assert_eq!(slots.end(), end * stride, "{context}");
                        if let Some(last) = count.checked_sub(1) {
                            assert_eq!(slots.index(last), Some(first + last), "{context}");
                            assert_eq!(slots.offset(last), Some((end - 1) * stride), "{context}");
                        }
                        assert_eq!(slots.index(count), None, "{context}");
                    }
                    Err(error) => {
                        overflowing += 1;
                        assert!(!every_index_fits, "{context}");
                        assert_eq!(error, HeapLayoutError::Overflow, "{context}");
                        assert_eq!(
                            plan.images(0).map(|slots| slots.first_index),
                            Ok(first),
                            "{context}: a rejected run leaves the plan unchanged"
                        );
                    }
                }
            }
        }
        assert!(fitting > 20 && overflowing > 10);
    }

    #[test]
    fn a_heap_that_ends_with_the_last_descriptor_holds_the_slices_of_the_run() {
        let mut heap = [0u8; 1];
        let one_byte_descriptor = DescriptorSlots {
            first_index: 0,
            count: 1,
            stride: 8,
            descriptor_size: 1,
        };
        assert!(one_byte_descriptor.bytes(&mut heap, 0).is_some());
        assert_eq!(
            one_byte_descriptor
                .slices(&mut heap)
                .map(|slices| slices.map(|slice| slice.len()).collect::<Vec<_>>()),
            Some(vec![1]),
            "the heap reaches the end of the last descriptor, not the end of its slot"
        );
    }

    #[test]
    fn plans_without_slots_or_reserved_range_have_size_0_and_cannot_be_bound_at_0() {
        let nothing_to_hold = vk::PhysicalDeviceDescriptorHeapPropertiesEXT::default()
            .buffer_descriptor_size(16)
            .buffer_descriptor_alignment(16)
            .image_descriptor_size(32)
            .image_descriptor_alignment(32)
            .sampler_descriptor_size(16)
            .sampler_descriptor_alignment(16);
        let mut empty_runs = ResourceHeapPlan::new(&nothing_to_hold);
        empty_runs.buffers(0).unwrap();
        empty_runs.images(0).unwrap();
        empty_runs.uniform(0).unwrap();
        let mut empty_samplers = SamplerHeapPlan::new(&nothing_to_hold);
        empty_samplers.samplers(0).unwrap();
        for layout in [
            ResourceHeapPlan::new(&nothing_to_hold).finish().unwrap(),
            SamplerHeapPlan::new(&nothing_to_hold).finish().unwrap(),
            empty_runs.finish().unwrap(),
            empty_samplers.finish().unwrap(),
        ] {
            assert_eq!(layout.size, 0, "a plan with no slots and no reserved range");
            assert_eq!(
                layout.bind_info(0).err(),
                Some(HeapLayoutError::NullAddress),
                "{layout:?}: no buffer has device address 0, whatever the size of the heap"
            );
        }
    }
}
