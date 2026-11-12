//! Byte layout of `VK_EXT_descriptor_heap` heaps.
//!
//! A heap is a buffer. Where its descriptors sit, how far apart, and where the range
//! reserved for the implementation goes, is arithmetic over
//! `VkPhysicalDeviceDescriptorHeapPropertiesEXT` that every user of the extension
//! otherwise writes by hand. This module is that arithmetic and nothing else: it holds
//! no Vulkan objects, allocates no slots and has no policy about reuse.
//!
//! ```
//! # use vulkan::vk;
//! # use vulkan_alloc::descriptor_heap::*;
//! # fn example(properties: &vk::PhysicalDeviceDescriptorHeapPropertiesEXT<'_>, heap_address: u64)
//! # -> Result<(), HeapLayoutError> {
//! let mut plan = ResourceHeapPlan::new(properties);
//! let buffers = plan.buffers(2)?; // two buffer descriptors
//! let images = plan.images(16)?; // sixteen image descriptors
//! let layout = plan.finish()?;
//!
//! // Create a buffer of `layout.size` bytes with DESCRIPTOR_HEAP_EXT usage, map it, and
//! // write descriptor `n` of a run into `run.bytes(mapped, n)`.
//! let mut mapped = vec![0u8; layout.size as usize];
//! let destination = images.bytes(&mut mapped, 3).expect("slot 3 exists");
//! assert_eq!(destination.len() as u64, properties.image_descriptor_size);
//!
//! // What a shader uses to reach that descriptor.
//! let shader_index = images.index(3).expect("slot 3 exists");
//! # let _ = (buffers, shader_index);
//!
//! // Checks the alignment of the address and fills in the reserved range.
//! let bind = layout.bind_info(heap_address)?;
//! # let _ = bind;
//! # Ok(()) }
//! ```
//!
//! # Indices
//!
//! A shader reaches a descriptor through an index, and the byte offset of the descriptor
//! is `index * stride`. What the stride is depends on how the shader addresses the heap.
//!
//! - With untyped pointers (Slang's `DescriptorHandle<T>`, SPIR-V `OpConstantSizeOfEXT`)
//!   the stride is the size of the descriptor type: buffers are indexed in units of
//!   `bufferDescriptorSize` and images in units of `imageDescriptorSize`.
//!   [`ResourceHeapPlan::buffers`] and [`ResourceHeapPlan::images`] make runs for this.
//! - With set-and-binding mappings (`VkDescriptorSetAndBindingMappingEXT`) the
//!   application chooses the stride and passes it as `heapIndexStride`.
//!   [`ResourceHeapPlan::uniform`] makes a run whose slots can each hold an image or a
//!   buffer descriptor.
//!
//! Either way the first slot of a run is at `first_index * stride`, so
//! [`DescriptorSlots::index`] is the number to hand to the shader.

use core::fmt;
use core::ops::Range;
use vulkan::vk;

/// Why a heap cannot be laid out or bound as asked.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeapLayoutError {
    /// The heap would be larger than `maxResourceHeapSize` / `maxSamplerHeapSize`.
    TooLarge { size: u64, max: u64 },
    /// The address of the heap is not a multiple of `resourceHeapAlignment` /
    /// `samplerHeapAlignment`.
    Misaligned { address: u64, alignment: u64 },
    /// A size or an index does not fit its integer type.
    Overflow,
    /// The size of a descriptor type is not a multiple of its alignment, so consecutive
    /// descriptors indexed by size cannot all be aligned. No known implementation does
    /// this; use [`ResourceHeapPlan::uniform`] if one does.
    UnalignedDescriptorSize { size: u64, alignment: u64 },
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DescriptorSlots {
    /// The index of the first slot, in units of `stride`.
    pub first_index: u32,
    /// The number of slots.
    pub count: u32,
    /// The distance in bytes from one slot to the next.
    pub stride: u64,
    /// The number of bytes `vkWriteResourceDescriptorsEXT` /
    /// `vkWriteSamplerDescriptorsEXT` needs to write one descriptor. At most `stride`.
    pub descriptor_size: u64,
}

impl DescriptorSlots {
    /// The index a shader uses for slot `slot` of this run, or `None` if `slot` is not
    /// in the run.
    pub const fn index(&self, slot: u32) -> Option<u32> {
        if slot < self.count {
            self.first_index.checked_add(slot)
        } else {
            None
        }
    }

    /// The byte offset of slot `slot` from the start of the heap.
    pub const fn offset(&self, slot: u32) -> Option<u64> {
        match self.index(slot) {
            Some(index) => (index as u64).checked_mul(self.stride),
            None => None,
        }
    }

    /// The bytes of slot `slot` to write a descriptor into, as a range of the heap.
    pub fn range(&self, slot: u32) -> Option<Range<usize>> {
        let start = usize::try_from(self.offset(slot)?).ok()?;
        let end = start.checked_add(usize::try_from(self.descriptor_size).ok()?)?;
        Some(start..end)
    }

    /// The bytes of slot `slot` inside the mapped heap `heap`. `None` if `slot` is not
    /// in the run or `heap` is too short.
    pub fn bytes<'a>(&self, heap: &'a mut [u8], slot: u32) -> Option<&'a mut [u8]> {
        heap.get_mut(self.range(slot)?)
    }

    /// The bytes of every slot of the run inside the mapped heap `heap`, in slot order.
    /// `None` if `heap` is too short.
    ///
    /// The slices do not overlap, so they can be the destinations of one
    /// `vkWriteResourceDescriptorsEXT` / `vkWriteSamplerDescriptorsEXT` call that fills
    /// the whole run.
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

    /// The byte offset just past the last slot. Saturates instead of overflowing for a
    /// hand-made run that no heap could hold.
    pub const fn end(&self) -> u64 {
        (self.first_index as u64 + self.count as u64).saturating_mul(self.stride)
    }
}

/// Size and reserved range of a heap. Made by [`ResourceHeapPlan::finish`] or
/// [`SamplerHeapPlan::finish`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DescriptorHeapLayout {
    /// The size in bytes of the buffer to create.
    pub size: u64,
    /// What the device address of the heap must be a multiple of. A buffer created with
    /// `DESCRIPTOR_HEAP_EXT` usage is expected to report at least this alignment in its
    /// memory requirements; [`DescriptorHeapLayout::bind_info`] checks the result.
    pub alignment: u64,
    /// Where the range reserved for the implementation starts. Do not write there.
    pub reserved_offset: u64,
    /// The size of the reserved range.
    pub reserved_size: u64,
}

impl DescriptorHeapLayout {
    /// The `VkBindHeapInfoEXT` for a heap of this layout at device address `address`.
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

/// Lays runs of slots out one after another, then the reserved range.
#[derive(Debug, Clone)]
struct Plan {
    /// The first byte not used by a run yet.
    cursor: u64,
    /// What the reserved range offset must be a multiple of.
    reserved_alignment: u64,
    reserved_size: u64,
    heap_alignment: u64,
    max_size: u64,
}

impl Plan {
    /// Places `count` slots of `stride` bytes, each aligned to `alignment`.
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
        // The first index whose offset `index * stride` is not before the cursor. Every
        // such offset is aligned because the stride is a multiple of the alignment.
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

/// Plans a resource heap: buffer and image descriptors.
#[derive(Debug, Clone)]
pub struct ResourceHeapPlan {
    plan: Plan,
    buffer_size: u64,
    buffer_alignment: u64,
    image_size: u64,
    image_alignment: u64,
}

impl ResourceHeapPlan {
    pub fn new(properties: &vk::PhysicalDeviceDescriptorHeapPropertiesEXT<'_>) -> Self {
        Self {
            plan: Plan {
                cursor: 0,
                // VUID-vkCmdBindResourceHeapEXT-pBindInfo-11435 and -11436.
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

    /// A run of buffer descriptors, indexed in units of `bufferDescriptorSize`.
    ///
    /// This is the indexing of a shader that reaches buffers through untyped pointers.
    pub fn buffers(&mut self, count: u32) -> Result<DescriptorSlots, HeapLayoutError> {
        self.plan.run(
            count,
            self.buffer_size,
            self.buffer_size,
            self.buffer_alignment,
        )
    }

    /// A run of image descriptors, indexed in units of `imageDescriptorSize`.
    ///
    /// This is the indexing of a shader that reaches images through untyped pointers.
    pub fn images(&mut self, count: u32) -> Result<DescriptorSlots, HeapLayoutError> {
        self.plan.run(
            count,
            self.image_size,
            self.image_size,
            self.image_alignment,
        )
    }

    /// A run of slots that can each hold an image descriptor or a buffer descriptor.
    ///
    /// The stride is the larger of the two descriptor sizes, rounded up to both
    /// alignments. Pass it as `heapIndexStride` of a set-and-binding mapping.
    /// `descriptor_size` of the result is the larger size; a buffer descriptor written
    /// into a slot uses a prefix of it.
    pub fn uniform(&mut self, count: u32) -> Result<DescriptorSlots, HeapLayoutError> {
        let alignment = lcm(self.buffer_alignment, self.image_alignment);
        let size = self.buffer_size.max(self.image_size);
        let stride = size
            .checked_next_multiple_of(alignment)
            .ok_or(HeapLayoutError::Overflow)?;
        self.plan.run(count, stride, size, alignment)
    }

    /// Places the reserved range after the last run and returns the heap layout.
    pub fn finish(self) -> Result<DescriptorHeapLayout, HeapLayoutError> {
        self.plan.finish()
    }
}

/// Plans a sampler heap.
#[derive(Debug, Clone)]
pub struct SamplerHeapPlan {
    plan: Plan,
    sampler_size: u64,
    sampler_alignment: u64,
}

impl SamplerHeapPlan {
    pub fn new(properties: &vk::PhysicalDeviceDescriptorHeapPropertiesEXT<'_>) -> Self {
        Self {
            plan: Plan {
                cursor: 0,
                // VUID-vkCmdBindSamplerHeapEXT-pBindInfo-11434.
                reserved_alignment: properties.sampler_descriptor_alignment,
                reserved_size: properties.min_sampler_heap_reserved_range,
                heap_alignment: properties.sampler_heap_alignment,
                max_size: properties.max_sampler_heap_size,
            },
            sampler_size: properties.sampler_descriptor_size,
            sampler_alignment: properties.sampler_descriptor_alignment,
        }
    }

    /// A run of sampler descriptors, indexed in units of `samplerDescriptorSize`.
    pub fn samplers(&mut self, count: u32) -> Result<DescriptorSlots, HeapLayoutError> {
        self.plan.run(
            count,
            self.sampler_size,
            self.sampler_size,
            self.sampler_alignment,
        )
    }

    /// Places the reserved range after the last run and returns the heap layout.
    pub fn finish(self) -> Result<DescriptorHeapLayout, HeapLayoutError> {
        self.plan.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Values RADV reports (Mesa 26.2).
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

    /// An implementation with a reserved range and sizes that are not powers of two.
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
            // Offsets are index * stride, aligned, and the runs are in order.
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
        // max(72, 48) rounded up to lcm(24, 16) = 48.
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
        // A heap that was mapped too short is an error value, not a panic.
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

        // A heap that is too short, and a hand-made run that makes no sense.
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
