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
    /// The address of the heap is 0 although its size is not.
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
    /// larger than `stride`. When `descriptor_size` is at most `stride`, as in every run
    /// the plans make, each slice is the one [`DescriptorSlots::bytes`] returns for its
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
    /// - [`HeapLayoutError::NullAddress`] if `address` is 0 and `size` is not.
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
        if address == 0 && self.size != 0 {
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
    /// - [`HeapLayoutError::Overflow`] if the least common multiple of the two
    ///   descriptor alignments or the stride does not fit in `u64`, the first index of
    ///   the run or the index of one of its slots does not fit in `u32`, or its end does
    ///   not fit in `u64`. A run of 0 slots has a first index too: where it starts, in
    ///   units of the stride.
    /// - [`HeapLayoutError::ZeroDescriptorSize`] if `bufferDescriptorSize` or
    ///   `imageDescriptorSize` is 0.
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

    /// Places the reserved range after the last run, at a multiple of both descriptor
    /// alignments, and returns the layout of the heap.
    ///
    /// # Errors
    ///
    /// - [`HeapLayoutError::TooLarge`] if the heap is larger than `maxResourceHeapSize`.
    /// - [`HeapLayoutError::Overflow`] if its size, or the least common multiple of the
    ///   two descriptor alignments, does not fit in `u64`.
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

    /// Places the reserved range after the last run, at a multiple of
    /// `samplerDescriptorAlignment`, and returns the layout of the heap.
    ///
    /// # Errors
    ///
    /// - [`HeapLayoutError::TooLarge`] if the heap is larger than `maxSamplerHeapSize`.
    /// - [`HeapLayoutError::Overflow`] if its size does not fit in `u64`.
    pub fn finish(self) -> Result<DescriptorHeapLayout, HeapLayoutError> {
        self.plan.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn radv() -> vk::PhysicalDeviceDescriptorHeapPropertiesEXT<'static> {
        vk::PhysicalDeviceDescriptorHeapPropertiesEXT::default()
            .sampler_heap_alignment(16)
            .resource_heap_alignment(32)
            .max_sampler_heap_size(1 << 20)
            .max_resource_heap_size(1 << 30)
            .min_sampler_heap_reserved_range(0)
            .min_resource_heap_reserved_range(0)
            .sampler_descriptor_size(16)
            .image_descriptor_size(32)
            .buffer_descriptor_size(16)
            .sampler_descriptor_alignment(16)
            .image_descriptor_alignment(32)
            .buffer_descriptor_alignment(16)
    }

    fn awkward() -> vk::PhysicalDeviceDescriptorHeapPropertiesEXT<'static> {
        vk::PhysicalDeviceDescriptorHeapPropertiesEXT::default()
            .sampler_heap_alignment(64)
            .resource_heap_alignment(4096)
            .max_sampler_heap_size(4096)
            .max_resource_heap_size(1 << 16)
            .min_sampler_heap_reserved_range(96)
            .min_resource_heap_reserved_range(1024)
            .sampler_descriptor_size(24)
            .image_descriptor_size(72)
            .buffer_descriptor_size(48)
            .sampler_descriptor_alignment(8)
            .image_descriptor_alignment(24)
            .buffer_descriptor_alignment(16)
    }

    #[test]
    fn typed_runs_do_not_overlap_and_index_in_units_of_their_own_size() {
        for properties in [radv(), awkward()] {
            let mut plan = ResourceHeapPlan::new(&properties);
            let buffers = plan.buffers(3).unwrap();
            let images = plan.images(2).unwrap();
            let more_buffers = plan.buffers(1).unwrap();
            let layout = plan.finish().unwrap();

            assert_eq!(buffers.first_index, 0);
            assert_eq!(buffers.stride, properties.buffer_descriptor_size);
            assert_eq!(images.stride, properties.image_descriptor_size);
            let mut previous_end = 0;
            for run in [buffers, images, more_buffers] {
                let alignment = if run.stride == properties.image_descriptor_size {
                    properties.image_descriptor_alignment
                } else {
                    properties.buffer_descriptor_alignment
                };
                for slot in 0..run.count {
                    let offset = run.offset(slot).unwrap();
                    assert_eq!(offset, u64::from(run.index(slot).unwrap()) * run.stride);
                    assert!(offset.is_multiple_of(alignment));
                    assert!(offset >= previous_end);
                    previous_end = offset + run.descriptor_size;
                }
                assert_eq!(run.index(run.count), None);
                assert_eq!(run.offset(run.count), None);
            }
            assert!(layout.reserved_offset >= previous_end);
            assert!(
                layout
                    .reserved_offset
                    .is_multiple_of(properties.buffer_descriptor_alignment)
            );
            assert!(
                layout
                    .reserved_offset
                    .is_multiple_of(properties.image_descriptor_alignment)
            );
            assert_eq!(
                layout.reserved_size,
                properties.min_resource_heap_reserved_range
            );
            assert_eq!(layout.size, layout.reserved_offset + layout.reserved_size);
            assert_eq!(layout.alignment, properties.resource_heap_alignment);
        }
    }

    #[test]
    fn uniform_slots_hold_either_kind() {
        let properties = awkward();
        let mut plan = ResourceHeapPlan::new(&properties);
        let slots = plan.uniform(8).unwrap();
        assert_eq!(slots.stride, 96);
        assert_eq!(slots.descriptor_size, 72);
        assert_eq!(slots.offset(7), Some(7 * 96));
        let layout = plan.finish().unwrap();
        assert_eq!(layout.reserved_offset, 8 * 96);
        assert_eq!(layout.size, 8 * 96 + 1024);
    }

    #[test]
    fn rejected_descriptor_sizes_are_reported_as_the_device_reports_them() {
        let values = [
            0,
            1,
            2,
            3,
            8,
            16,
            24,
            32,
            40,
            48,
            64,
            72,
            96,
            128,
            1 << 32,
            1 << 63,
            u64::MAX - 1,
            u64::MAX,
        ];
        let mut unaligned = 0;
        let mut zero = 0;
        for size in values {
            for alignment in values {
                for kind in 0..3 {
                    let context = format!("kind {kind}, size {size}, alignment {alignment}");
                    let pair = |of: usize, valid: u64| {
                        if of == kind {
                            (size, alignment)
                        } else {
                            (valid, valid)
                        }
                    };
                    let (buffer_size, buffer_alignment) = pair(0, 16);
                    let (image_size, image_alignment) = pair(1, 32);
                    let (sampler_size, sampler_alignment) = pair(2, 8);
                    let properties = vk::PhysicalDeviceDescriptorHeapPropertiesEXT::default()
                        .max_resource_heap_size(u64::MAX)
                        .max_sampler_heap_size(u64::MAX)
                        .buffer_descriptor_size(buffer_size)
                        .buffer_descriptor_alignment(buffer_alignment)
                        .image_descriptor_size(image_size)
                        .image_descriptor_alignment(image_alignment)
                        .sampler_descriptor_size(sampler_size)
                        .sampler_descriptor_alignment(sampler_alignment);
                    let expected = if size == 0 {
                        zero += 1;
                        Err(HeapLayoutError::ZeroDescriptorSize)
                    } else if size.is_multiple_of(alignment.max(1)) {
                        Ok(size)
                    } else {
                        unaligned += 1;
                        Err(HeapLayoutError::UnalignedDescriptorSize { size, alignment })
                    };

                    let mut resources = ResourceHeapPlan::new(&properties);
                    let mut samplers = SamplerHeapPlan::new(&properties);
                    let run = match kind {
                        0 => resources.buffers(1),
                        1 => resources.images(1),
                        _ => samplers.samplers(1),
                    };
                    assert_eq!(run.map(|slots| slots.stride), expected, "{context}");
                    if let Err(HeapLayoutError::UnalignedDescriptorSize { size, alignment }) = run {
                        assert!(
                            size != 0 && !size.is_multiple_of(alignment.max(1)),
                            "{context}"
                        );
                    }

                    if kind != 2 {
                        let uniform = ResourceHeapPlan::new(&properties).uniform(1);
                        assert!(
                            !matches!(
                                uniform,
                                Err(HeapLayoutError::UnalignedDescriptorSize { .. })
                            ),
                            "{context}: {uniform:?}"
                        );
                        assert_eq!(
                            uniform == Err(HeapLayoutError::ZeroDescriptorSize),
                            size == 0,
                            "{context}: {uniform:?}"
                        );
                        if let Ok(slots) = uniform {
                            assert!(slots.stride >= slots.descriptor_size, "{context}");
                            assert_eq!(slots.descriptor_size, buffer_size.max(image_size));
                        }
                    }
                }
            }
        }
        assert!(unaligned > 100 && zero > 10);

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
    fn bytes_are_exactly_one_descriptor_and_stay_inside_the_heap() {
        let properties = radv();
        let mut plan = ResourceHeapPlan::new(&properties);
        let buffers = plan.buffers(1).unwrap();
        let images = plan.images(2).unwrap();
        let layout = plan.finish().unwrap();
        let mut heap = vec![0u8; layout.size as usize];

        assert_eq!(buffers.range(0), Some(0..16));
        assert_eq!(images.first_index, 1);
        assert_eq!(images.range(1), Some(64..96));
        images.bytes(&mut heap, 1).unwrap().fill(0xff);
        assert!(heap[..64].iter().all(|byte| *byte == 0));
        assert!(heap[64..96].iter().all(|byte| *byte == 0xff));

        assert!(images.bytes(&mut heap, 2).is_none());
        assert!(images.bytes(&mut heap[..80], 1).is_none());
    }

    #[test]
    fn sampler_heap_reserves_after_the_samplers() {
        let properties = awkward();
        let mut plan = SamplerHeapPlan::new(&properties);
        let samplers = plan.samplers(5).unwrap();
        let layout = plan.finish().unwrap();
        assert_eq!(samplers.stride, 24);
        assert_eq!(samplers.end(), 120);
        assert_eq!(layout.reserved_offset, 120);
        assert_eq!(layout.reserved_size, 96);
        assert_eq!(layout.size, 216);
        assert_eq!(layout.alignment, 64);
    }

    #[test]
    fn embedded_samplers_reserve_the_larger_of_the_two_reserved_ranges() {
        let mut seed = 0x2545_f491_4f6c_dd1d_u64;
        let mut next = move || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed
        };
        let mut reserved = |choice: u64| match choice % 5 {
            0 => 0,
            1 => next() % 4096,
            2 => next() % (1 << 20),
            3 => next(),
            _ => u64::MAX - next() % 4096,
        };
        let mut outcomes = [0; 3];
        for case in 0..5000_u64 {
            let without = reserved(case);
            let with = reserved(case / 5);
            let sampler_size = 1 << (case % 7);
            let alignment = (sampler_size >> (case % 3)).max(1);
            let count = (case * 7919 % 4001) as u32;
            let max = match case % 3 {
                0 => u64::MAX,
                1 => 1 << 20,
                _ => 1 << 16,
            };
            let properties = vk::PhysicalDeviceDescriptorHeapPropertiesEXT::default()
                .sampler_heap_alignment(64)
                .max_sampler_heap_size(max)
                .min_sampler_heap_reserved_range(without)
                .min_sampler_heap_reserved_range_with_embedded(with)
                .sampler_descriptor_size(sampler_size)
                .sampler_descriptor_alignment(alignment);
            let context = format!(
                "{count} samplers of {sampler_size} bytes, reserved {without} or {with} \
                 with embedded samplers, at most {max} bytes"
            );
            let samplers_end = u128::from(count) * u128::from(sampler_size);
            let expected = |reserved_size: u64| {
                let size = samplers_end + u128::from(reserved_size);
                match u64::try_from(size) {
                    Err(_) => Err(HeapLayoutError::Overflow),
                    Ok(size) if size > max => Err(HeapLayoutError::TooLarge { size, max }),
                    Ok(size) => Ok((samplers_end as u64, reserved_size, size)),
                }
            };
            let layout = |mut plan: SamplerHeapPlan| {
                plan.samplers(count).unwrap();
                plan.finish().map(|layout| {
                    assert_eq!(layout.alignment, 64);
                    (layout.reserved_offset, layout.reserved_size, layout.size)
                })
            };

            let embedded = layout(SamplerHeapPlan::with_embedded_samplers(&properties));
            outcomes[match embedded {
                Ok(_) => 0,
                Err(HeapLayoutError::TooLarge { .. }) => 1,
                Err(_) => 2,
            }] += 1;
            assert_eq!(embedded, expected(without.max(with)), "{context}");
            assert_eq!(
                layout(SamplerHeapPlan::new(&properties)),
                expected(without),
                "{context}"
            );
        }
        assert!(
            outcomes.iter().all(|&count| count > 100),
            "every outcome is exercised: {outcomes:?}"
        );
    }

    #[test]
    fn limits_are_errors_not_panics() {
        let properties = awkward();
        let mut plan = SamplerHeapPlan::new(&properties);
        plan.samplers(1000).unwrap();
        assert_eq!(
            plan.finish(),
            Err(HeapLayoutError::TooLarge {
                size: 24_096,
                max: 4096
            })
        );

        let mut plan = ResourceHeapPlan::new(&properties);
        plan.images(u32::MAX).unwrap();
        assert_eq!(
            plan.images(1).map(|slots| slots.index(0)),
            Ok(Some(u32::MAX))
        );
        assert_eq!(plan.images(0), Err(HeapLayoutError::Overflow));
        assert_eq!(plan.images(1), Err(HeapLayoutError::Overflow));

        let odd = radv().image_descriptor_size(40);
        assert_eq!(
            ResourceHeapPlan::new(&odd).images(1),
            Err(HeapLayoutError::UnalignedDescriptorSize {
                size: 40,
                alignment: 32
            })
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
        let properties = awkward();
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
    fn descriptor_alignments_of_any_size_are_laid_out_or_rejected() {
        let mut seed = 0x9e37_79b9_7f4a_7c15_u64;
        let mut next = move || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed
        };
        let mut alignment = |random: u64| match random % 6 {
            0 => 1 << (next() % 64),
            1 => next(),
            2 => next() >> 32,
            3 => (next() >> 31) | 1,
            4 => u64::MAX - next() % 4,
            _ => next() % 64,
        };
        let mut overflowing = 0;
        let mut fitting = 0;
        for case in 0..4000_u64 {
            let buffer_alignment = alignment(case);
            let image_alignment = alignment(case / 6);
            let properties = vk::PhysicalDeviceDescriptorHeapPropertiesEXT::default()
                .max_resource_heap_size(u64::MAX)
                .buffer_descriptor_size(16)
                .image_descriptor_size(32)
                .buffer_descriptor_alignment(buffer_alignment)
                .image_descriptor_alignment(image_alignment);
            let (a, b) = (
                u128::from(buffer_alignment.max(1)),
                u128::from(image_alignment.max(1)),
            );
            let gcd = {
                let (mut a, mut b) = (a, b);
                while b != 0 {
                    (a, b) = (b, a % b);
                }
                a
            };
            let both = u64::try_from(a / gcd * b).ok();
            let context = format!("alignments {buffer_alignment} and {image_alignment}");

            let mut plan = ResourceHeapPlan::new(&properties);
            let uniform = plan.uniform(1);
            let mut after_one_byte = ResourceHeapPlan::new(&properties);
            after_one_byte.plan.cursor = 1;
            let layout = after_one_byte.finish();
            match both {
                None => {
                    overflowing += 1;
                    assert_eq!(uniform, Err(HeapLayoutError::Overflow), "{context}");
                    assert_eq!(layout, Err(HeapLayoutError::Overflow), "{context}");
                    assert_eq!(plan.finish(), Err(HeapLayoutError::Overflow), "{context}");
                }
                Some(both) => {
                    fitting += 1;
                    let stride = 32_u64.checked_next_multiple_of(both);
                    assert_eq!(uniform.map(|slots| slots.stride).ok(), stride, "{context}");
                    assert_eq!(
                        layout.map(|layout| layout.reserved_offset),
                        Ok(both),
                        "{context}"
                    );
                }
            }
        }
        assert!(overflowing > 100 && fitting > 100);
    }

    #[test]
    fn slices_cover_each_slot_once() {
        let properties = awkward();
        let mut plan = ResourceHeapPlan::new(&properties);
        plan.buffers(1).unwrap();
        let slots = plan.uniform(3).unwrap();
        let layout = plan.finish().unwrap();
        let mut heap = vec![0u8; layout.size as usize];

        for (slot, bytes) in slots.slices(&mut heap).unwrap().enumerate() {
            assert_eq!(bytes.len() as u64, slots.descriptor_size);
            bytes.fill(slot as u8 + 1);
        }
        for slot in 0..3 {
            let range = slots.range(slot).unwrap();
            assert!(heap[range].iter().all(|&byte| byte == slot as u8 + 1));
        }
        let written = heap.iter().filter(|&&byte| byte != 0).count() as u64;
        assert_eq!(written, 3 * slots.descriptor_size);

        assert!(slots.slices(&mut heap[..10]).is_none());
        let broken = DescriptorSlots {
            first_index: u32::MAX,
            count: u32::MAX,
            stride: 0,
            descriptor_size: u64::MAX,
        };
        assert!(broken.slices(&mut heap).is_none());
        let huge = DescriptorSlots {
            stride: u64::MAX,
            ..broken
        };
        assert_eq!(huge.end(), u64::MAX);
        assert!(huge.slices(&mut heap).is_none());
        let empty = DescriptorSlots { count: 0, ..slots };
        assert_eq!(empty.slices(&mut heap).unwrap().count(), 0);
    }

    #[test]
    fn slices_accept_exactly_the_heaps_that_bytes_accepts_for_every_slot() {
        let mut seed = 0xd1b5_4a32_d192_ed03_u64;
        let mut next = move || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed
        };
        let mut buffer = vec![0u8; 1 << 16];
        let one_byte_descriptor = DescriptorSlots {
            first_index: 0,
            count: 1,
            stride: 8,
            descriptor_size: 1,
        };
        assert!(
            one_byte_descriptor.bytes(&mut buffer[..1], 0).is_some()
                && one_byte_descriptor.slices(&mut buffer[..1]).is_some(),
            "a heap that ends with the last descriptor holds the slices of the run"
        );
        let mut outcomes = [0; 3];
        for case in 0..20_000_u64 {
            let stride = match case % 7 {
                0 => 1 + next() % 8,
                1 => [8, 16, 24, 32, 48, 72, 96, 256][(next() % 8) as usize],
                2 => 1 + next() % 4096,
                3 => 1 << (next() % 64),
                4 => u64::MAX - next() % 4,
                5 => next() | 1,
                _ => 1,
            };
            let first_index = match next() % 6 {
                0 => 0,
                1 => 1,
                2 => (next() % 64) as u32,
                3 => (next() % 4096) as u32,
                4 => u32::MAX - (next() % 4) as u32,
                _ => next() as u32,
            };
            let count = (next() % 6) as u32;
            let descriptor_size = match next() % 5 {
                0 => 0,
                1 => 1,
                2 => stride,
                3 => stride - 1,
                _ => next() % stride,
            };
            let slots = DescriptorSlots {
                first_index,
                count,
                stride,
                descriptor_size,
            };
            let last_end = (u128::from(first_index) + u128::from(count.saturating_sub(1)))
                * u128::from(stride)
                + u128::from(descriptor_size);
            let heap_length = match next() % 5 {
                0 => last_end,
                1 => last_end.saturating_sub(1),
                2 => last_end + 1,
                3 => u128::from(slots.end()),
                _ => u128::from(next() % (1 << 16)),
            }
            .min(1 << 16) as usize;
            let heap = &mut buffer[..heap_length];
            let context = format!("{slots:?} in a heap of {heap_length} bytes");

            let expected: Option<Vec<(usize, usize)>> = (0..count)
                .map(|slot| {
                    slots
                        .bytes(heap, slot)
                        .map(|bytes| (bytes.as_ptr() as usize, bytes.len()))
                })
                .collect();
            let actual = slots.slices(heap).map(|slices| {
                slices
                    .map(|bytes| (bytes.as_ptr() as usize, bytes.len()))
                    .collect::<Vec<_>>()
            });
            outcomes[match (&actual, count) {
                (None, _) => 0,
                (Some(_), 0) => 1,
                (Some(_), _) => 2,
            }] += 1;
            assert_eq!(
                actual, expected,
                "{context}: slices is Some exactly when bytes is Some for every slot, with the same slices"
            );

            let wide = DescriptorSlots {
                descriptor_size: stride.saturating_add(1 + next() % 64),
                ..slots
            };
            let cut = DescriptorSlots {
                descriptor_size: stride,
                ..slots
            };
            let slices = |slots: DescriptorSlots, heap: &mut [u8]| {
                slots.slices(heap).map(|slices| {
                    slices
                        .map(|bytes| (bytes.as_ptr() as usize, bytes.len()))
                        .collect::<Vec<_>>()
                })
            };
            assert_eq!(
                slices(wide, heap),
                slices(cut, heap),
                "{context}: a descriptor larger than the stride is cut to the stride"
            );
        }
        assert!(
            outcomes.iter().all(|&count| count > 500),
            "every outcome is exercised: {outcomes:?}"
        );
    }

    #[test]
    fn bind_info_checks_the_address() {
        let properties = awkward();
        let mut plan = ResourceHeapPlan::new(&properties);
        plan.images(4).unwrap();
        let layout = plan.finish().unwrap();

        let bind = layout.bind_info(0x1_0000).unwrap();
        assert_eq!(bind.heap_range.address, 0x1_0000);
        assert_eq!(bind.heap_range.size, layout.size);
        assert_eq!(bind.reserved_range_offset, layout.reserved_offset);
        assert_eq!(bind.reserved_range_size, 1024);
        assert_eq!(
            layout.bind_info(0x1_0040).err(),
            Some(HeapLayoutError::Misaligned {
                address: 0x1_0040,
                alignment: 4096
            })
        );
    }

    #[test]
    fn bind_info_accepts_exactly_the_multiples_of_the_alignment_with_0_as_1() {
        let alignments = [0, 1, 2, 3, 16, 24, 4096, 65536, 1 << 63, u64::MAX];
        let addresses = [
            0,
            1,
            2,
            3,
            16,
            24,
            48,
            0x1_0000,
            0x1_0040,
            0xdead_beef,
            1 << 63,
            u64::MAX - 1,
            u64::MAX,
        ];
        let mut accepted = 0;
        let mut rejected = 0;
        for alignment in alignments {
            let layout = DescriptorHeapLayout {
                size: 0x2_0000,
                alignment,
                reserved_offset: 0x1_f000,
                reserved_size: 0x1000,
            };
            for address in addresses {
                let context = format!("address {address:#x}, alignment {alignment}");
                let inside_the_address_space =
                    address != 0 && u128::from(address) + u128::from(layout.size) <= 1 << 64;
                match layout.bind_info(address) {
                    Ok(bind) => {
                        accepted += 1;
                        assert!(address.is_multiple_of(alignment.max(1)), "{context}");
                        assert!(inside_the_address_space, "{context}");
                        assert_eq!(bind.heap_range.address, address, "{context}");
                        assert_eq!(bind.heap_range.size, layout.size, "{context}");
                        assert_eq!(
                            bind.reserved_range_offset, layout.reserved_offset,
                            "{context}"
                        );
                        assert_eq!(bind.reserved_range_size, layout.reserved_size, "{context}");
                    }
                    Err(HeapLayoutError::Misaligned {
                        address: rejected_address,
                        alignment: required,
                    }) => {
                        rejected += 1;
                        assert!(!address.is_multiple_of(alignment.max(1)), "{context}");
                        assert_eq!((rejected_address, required), (address, alignment));
                    }
                    Err(error) => {
                        assert!(address.is_multiple_of(alignment.max(1)), "{context}");
                        assert!(!inside_the_address_space, "{context}: {error:?}");
                    }
                }
            }
        }
        assert!(accepted > 20 && rejected > 20);
    }

    #[test]
    fn bind_info_rejects_exactly_the_ranges_the_device_must_not_be_given() {
        let sizes = [0, 1, 0x100, 0x1000, 0x2_0000, 1 << 63, u64::MAX];
        let reserved_offsets = [0, 0x100, 0x1000, 0x1_f000, 1 << 63, u64::MAX];
        let reserved_sizes = [0, 0x100, 0x1000, u64::MAX];
        let alignments = [0, 1, 0x100, 4096];
        let addresses = [
            0,
            1,
            0x100,
            0x1000,
            0x1_0000,
            1 << 63,
            u64::MAX - 0x1_ffff,
            u64::MAX - 0xfff,
            u64::MAX - 0xff,
            u64::MAX,
        ];
        let mut outcomes = [0; 5];
        for size in sizes {
            for reserved_offset in reserved_offsets {
                for reserved_size in reserved_sizes {
                    for alignment in alignments {
                        let layout = DescriptorHeapLayout {
                            size,
                            alignment,
                            reserved_offset,
                            reserved_size,
                        };
                        for address in addresses {
                            let expected = if u128::from(reserved_offset)
                                + u128::from(reserved_size)
                                > u128::from(size)
                            {
                                outcomes[0] += 1;
                                Err(HeapLayoutError::ReservedRangeOutsideHeap {
                                    reserved_offset,
                                    reserved_size,
                                    heap_size: size,
                                })
                            } else if u128::from(address) % u128::from(alignment.max(1)) != 0 {
                                outcomes[1] += 1;
                                Err(HeapLayoutError::Misaligned { address, alignment })
                            } else if address == 0 && size != 0 {
                                outcomes[2] += 1;
                                Err(HeapLayoutError::NullAddress)
                            } else if u128::from(address) + u128::from(size) > 1 << 64 {
                                outcomes[3] += 1;
                                Err(HeapLayoutError::AddressRangeOverflow { address, size })
                            } else {
                                outcomes[4] += 1;
                                Ok((address, size, reserved_offset, reserved_size))
                            };
                            let bind = layout.bind_info(address).map(|bind| {
                                (
                                    bind.heap_range.address,
                                    bind.heap_range.size,
                                    bind.reserved_range_offset,
                                    bind.reserved_range_size,
                                )
                            });
                            assert_eq!(bind, expected, "{layout:?} at {address:#x}");
                        }
                    }
                }
            }
        }
        assert!(
            outcomes.iter().all(|&count| count > 50),
            "every outcome is exercised: {outcomes:?}"
        );
    }

    fn xorshift(mut seed: u64) -> impl FnMut() -> u64 {
        move || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed
        }
    }

    fn random_run(
        resources: &mut ResourceHeapPlan,
        samplers: &mut SamplerHeapPlan,
        kind: u64,
        count: u32,
    ) -> Result<DescriptorSlots, HeapLayoutError> {
        match kind % 4 {
            0 => resources.buffers(count),
            1 => resources.images(count),
            2 => resources.uniform(count),
            _ => samplers.samplers(count),
        }
    }

    #[test]
    fn a_run_of_0_slots_moves_the_plan_to_a_multiple_of_its_stride_if_that_fits() {
        let mut next = xorshift(0x6a09_e667_f3bc_c908);
        let sizes = [
            1,
            3,
            8,
            16,
            24,
            48,
            72,
            96,
            1 << 20,
            1 << 32,
            1 << 63,
            (1 << 63) + 1,
            u64::MAX,
        ];
        let mut outcomes = [0; 4];
        for _ in 0..5000 {
            let mut pick = || sizes[(next() % sizes.len() as u64) as usize];
            let (buffer_size, image_size, sampler_size) = (pick(), pick(), pick());
            let properties = vk::PhysicalDeviceDescriptorHeapPropertiesEXT::default()
                .max_resource_heap_size(u64::MAX)
                .max_sampler_heap_size(u64::MAX)
                .buffer_descriptor_size(buffer_size)
                .buffer_descriptor_alignment(1)
                .image_descriptor_size(image_size)
                .image_descriptor_alignment(1)
                .sampler_descriptor_size(sampler_size)
                .sampler_descriptor_alignment(1);
            let mut resources = ResourceHeapPlan::new(&properties);
            let mut samplers = SamplerHeapPlan::new(&properties);
            for _ in 0..6 {
                let kind = next() % 4;
                let count = match next() % 4 {
                    0 | 1 => 0,
                    2 => (next() % 4) as u32,
                    _ => next() as u32,
                };
                let stride = match kind {
                    0 => buffer_size,
                    1 => image_size,
                    2 => buffer_size.max(image_size),
                    _ => sampler_size,
                };
                let cursor = |resources: &ResourceHeapPlan, samplers: &SamplerHeapPlan| {
                    if kind == 3 {
                        samplers.plan.cursor
                    } else {
                        resources.plan.cursor
                    }
                };
                let before = cursor(&resources, &samplers);
                let run = random_run(&mut resources, &mut samplers, kind, count);
                let after = cursor(&resources, &samplers);
                if count != 0 {
                    continue;
                }
                let context = format!("a run of 0 slots of stride {stride} at byte {before}");
                let first_index = u128::from(before).div_ceil(u128::from(stride));
                let start = first_index * u128::from(stride);
                if first_index > u128::from(u32::MAX) || start > u128::from(u64::MAX) {
                    outcomes[if first_index > u128::from(u32::MAX) {
                        2
                    } else {
                        3
                    }] += 1;
                    assert_eq!(run, Err(HeapLayoutError::Overflow), "{context}");
                    assert_eq!(after, before, "{context}: a rejected run leaves the plan");
                } else {
                    outcomes[usize::from(after != before)] += 1;
                    assert_eq!(
                        run,
                        Ok(DescriptorSlots {
                            first_index: first_index as u32,
                            count: 0,
                            stride,
                            descriptor_size: stride,
                        }),
                        "{context}"
                    );
                    assert_eq!(
                        u128::from(after),
                        start,
                        "{context}: the plan moves up to the next multiple of the stride"
                    );
                }
            }
        }
        assert!(
            outcomes.iter().all(|&count| count > 20),
            "every outcome is exercised: {outcomes:?}"
        );
    }

    #[test]
    fn a_descriptor_alignment_of_0_lays_out_heaps_as_an_alignment_of_1() {
        let mut next = xorshift(0xbb67_ae85_84ca_a73b);
        let sizes = [0, 1, 3, 8, 16, 24, 32, 48, 72, 96, 1 << 32, u64::MAX];
        let alignments = [0, 0, 0, 1, 2, 8, 16, 24, 32];
        let mut with_0 = 0;
        let mut laid_out = 0;
        for _ in 0..5000 {
            let mut size = || sizes[(next() % sizes.len() as u64) as usize];
            let (buffer_size, image_size, sampler_size) = (size(), size(), size());
            let mut alignment = || alignments[(next() % alignments.len() as u64) as usize];
            let (buffer_alignment, image_alignment, sampler_alignment) =
                (alignment(), alignment(), alignment());
            let properties = |zero_as: u64| {
                let alignment = |alignment: u64| if alignment == 0 { zero_as } else { alignment };
                vk::PhysicalDeviceDescriptorHeapPropertiesEXT::default()
                    .resource_heap_alignment(64)
                    .sampler_heap_alignment(32)
                    .max_resource_heap_size(1 << 40)
                    .max_sampler_heap_size(1 << 40)
                    .min_resource_heap_reserved_range(96)
                    .min_sampler_heap_reserved_range(48)
                    .buffer_descriptor_size(buffer_size)
                    .buffer_descriptor_alignment(alignment(buffer_alignment))
                    .image_descriptor_size(image_size)
                    .image_descriptor_alignment(alignment(image_alignment))
                    .sampler_descriptor_size(sampler_size)
                    .sampler_descriptor_alignment(alignment(sampler_alignment))
            };
            if [buffer_alignment, image_alignment, sampler_alignment].contains(&0) {
                with_0 += 1;
            }
            let (zero, one) = (properties(0), properties(1));
            let context = format!(
                "descriptor sizes {buffer_size}, {image_size} and {sampler_size}, alignments \
                 {buffer_alignment}, {image_alignment} and {sampler_alignment}"
            );
            let mut resources = [ResourceHeapPlan::new(&zero), ResourceHeapPlan::new(&one)];
            let mut samplers = [SamplerHeapPlan::new(&zero), SamplerHeapPlan::new(&one)];
            for _ in 0..6 {
                let kind = next() % 4;
                let count = match next() % 3 {
                    0 => 0,
                    1 => (next() % 8) as u32,
                    _ => next() as u32,
                };
                let [resources_0, resources_1] = &mut resources;
                let [samplers_0, samplers_1] = &mut samplers;
                assert_eq!(
                    random_run(resources_0, samplers_0, kind, count),
                    random_run(resources_1, samplers_1, kind, count),
                    "{context}: run {kind} of {count} slots"
                );
            }
            let [resources_0, resources_1] = resources;
            let resource_layout = resources_0.finish();
            assert_eq!(resource_layout, resources_1.finish(), "{context}");
            let [samplers_0, samplers_1] = samplers;
            let sampler_layout = samplers_0.finish();
            assert_eq!(sampler_layout, samplers_1.finish(), "{context}");
            if resource_layout.is_ok() && sampler_layout.is_ok() {
                laid_out += 1;
            }
        }
        assert!(
            with_0 > 1000 && laid_out > 500,
            "{with_0} cases with an alignment of 0, {laid_out} laid out"
        );
    }
}
