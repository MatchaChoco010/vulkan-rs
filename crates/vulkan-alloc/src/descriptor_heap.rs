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
    /// An index does not fit in `u32` or a byte offset or size does not fit in `u64`.
    Overflow,
    /// The size of a descriptor type is not a multiple of its alignment, so descriptors
    /// indexed in units of their size cannot all be aligned.
    UnalignedDescriptorSize {
        /// The descriptor size, in bytes.
        size: u64,
        /// The descriptor alignment, in bytes.
        alignment: u64,
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

    /// The bytes of every slot of the run inside the mapped heap `heap`, in slot order,
    /// each `descriptor_size` bytes long (at most `stride`).
    ///
    /// The slices do not overlap, so they can all be destinations of one
    /// `vkWriteResourceDescriptorsEXT` or `vkWriteSamplerDescriptorsEXT` call that fills
    /// the whole run. `None` if `heap` is too short, `stride` is 0, or an offset does not
    /// fit in `usize`.
    pub fn slices<'a>(
        &self,
        heap: &'a mut [u8],
    ) -> Option<impl Iterator<Item = &'a mut [u8]> + use<'a>> {
        let stride = usize::try_from(self.stride)
            .ok()
            .filter(|&stride| stride != 0)?;
        let descriptor_size = usize::try_from(self.descriptor_size).ok()?.min(stride);
        let start = usize::try_from(self.first_index)
            .ok()?
            .checked_mul(stride)?;
        let length = usize::try_from(self.count).ok()?.checked_mul(stride)?;
        let run = heap.get_mut(start..start.checked_add(length)?)?;
        Some(
            run.chunks_exact_mut(stride)
                .map(move |slot| slot.split_at_mut(descriptor_size).0),
        )
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
    /// The size of the reserved range: `minResourceHeapReservedRange` or
    /// `minSamplerHeapReservedRange`.
    pub reserved_size: u64,
}

impl DescriptorHeapLayout {
    /// The [`vk::BindHeapInfoEXT`] for a heap of this layout at device address
    /// `address`, covering [`DescriptorHeapLayout::size`] bytes and naming the reserved
    /// range.
    ///
    /// # Errors
    ///
    /// [`HeapLayoutError::Misaligned`] if `address` is not a multiple of
    /// [`DescriptorHeapLayout::alignment`].
    pub fn bind_info(
        &self,
        address: vk::DeviceAddress,
    ) -> Result<vk::BindHeapInfoEXT<'static>, HeapLayoutError> {
        if !address.is_multiple_of(self.alignment) {
            return Err(HeapLayoutError::Misaligned {
                address,
                alignment: self.alignment,
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
    reserved_alignment: u64,
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
        let stride = stride.max(1);
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
        first_index
            .checked_add(count)
            .ok_or(HeapLayoutError::Overflow)?;
        self.cursor = (u64::from(first_index) + u64::from(count))
            .checked_mul(stride)
            .ok_or(HeapLayoutError::Overflow)?;
        Ok(slots)
    }

    fn finish(self) -> Result<DescriptorHeapLayout, HeapLayoutError> {
        let reserved_offset = self
            .cursor
            .checked_next_multiple_of(self.reserved_alignment.max(1))
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

fn lcm(a: u64, b: u64) -> u64 {
    fn gcd(mut a: u64, mut b: u64) -> u64 {
        while b != 0 {
            (a, b) = (b, a % b);
        }
        a
    }
    let (a, b) = (a.max(1), b.max(1));
    a / gcd(a, b) * b
}

/// Plans the layout of a resource heap, which holds buffer and image descriptors.
///
/// Each run starts at the first multiple of its stride at or after the end of the
/// previous run, so runs never overlap but gaps can appear between runs of different
/// strides. A method that returns an error leaves the plan unchanged.
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
    /// `maxResourceHeapSize` and `minResourceHeapReservedRange`.
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
    /// - [`HeapLayoutError::UnalignedDescriptorSize`] if `bufferDescriptorSize` is not a
    ///   multiple of `bufferDescriptorAlignment`.
    /// - [`HeapLayoutError::Overflow`] if an index of the run does not fit in `u32` or
    ///   its end does not fit in `u64`.
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
    /// - [`HeapLayoutError::UnalignedDescriptorSize`] if `imageDescriptorSize` is not a
    ///   multiple of `imageDescriptorAlignment`.
    /// - [`HeapLayoutError::Overflow`] if an index of the run does not fit in `u32` or
    ///   its end does not fit in `u64`.
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
    /// - [`HeapLayoutError::Overflow`] if the stride, an index of the run, or its end
    ///   does not fit.
    /// - [`HeapLayoutError::UnalignedDescriptorSize`] if both descriptor sizes are 0 and
    ///   an alignment is larger than 1.
    pub fn uniform(&mut self, count: u32) -> Result<DescriptorSlots, HeapLayoutError> {
        let alignment = lcm(self.buffer_alignment, self.image_alignment);
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
    /// - [`HeapLayoutError::Overflow`] if its size does not fit in `u64`.
    pub fn finish(self) -> Result<DescriptorHeapLayout, HeapLayoutError> {
        self.plan.finish()
    }
}

/// Plans the layout of a sampler heap.
///
/// Runs are placed one after another as in [`ResourceHeapPlan`]. A method that returns
/// an error leaves the plan unchanged.
#[derive(Debug, Clone)]
pub struct SamplerHeapPlan {
    plan: Plan,
    sampler_size: u64,
    sampler_alignment: u64,
}

impl SamplerHeapPlan {
    /// Starts an empty plan from the device's descriptor heap properties: the sampler
    /// descriptor size and alignment, `samplerHeapAlignment`, `maxSamplerHeapSize` and
    /// `minSamplerHeapReservedRange`.
    pub fn new(properties: &vk::PhysicalDeviceDescriptorHeapPropertiesEXT<'_>) -> Self {
        Self {
            plan: Plan {
                cursor: 0,
                reserved_alignment: properties.sampler_descriptor_alignment,
                reserved_size: properties.min_sampler_heap_reserved_range,
                heap_alignment: properties.sampler_heap_alignment,
                max_size: properties.max_sampler_heap_size,
            },
            sampler_size: properties.sampler_descriptor_size,
            sampler_alignment: properties.sampler_descriptor_alignment,
        }
    }

    /// Adds a run of `count` sampler descriptors, indexed in units of
    /// `samplerDescriptorSize`.
    ///
    /// # Errors
    ///
    /// - [`HeapLayoutError::UnalignedDescriptorSize`] if `samplerDescriptorSize` is not
    ///   a multiple of `samplerDescriptorAlignment`.
    /// - [`HeapLayoutError::Overflow`] if an index of the run does not fit in `u32` or
    ///   its end does not fit in `u64`.
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
}
