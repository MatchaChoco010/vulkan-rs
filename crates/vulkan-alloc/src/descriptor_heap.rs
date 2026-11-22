use core::fmt;
use core::ops::Range;
use vulkan::vk;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeapLayoutError {
    TooLarge { size: u64, max: u64 },
    Misaligned { address: u64, alignment: u64 },
    Overflow,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DescriptorSlots {
    pub first_index: u32,
    pub count: u32,
    pub stride: u64,
    pub descriptor_size: u64,
}

impl DescriptorSlots {
    pub const fn index(&self, slot: u32) -> Option<u32> {
        if slot < self.count {
            self.first_index.checked_add(slot)
        } else {
            None
        }
    }

    pub const fn offset(&self, slot: u32) -> Option<u64> {
        match self.index(slot) {
            Some(index) => (index as u64).checked_mul(self.stride),
            None => None,
        }
    }

    pub fn range(&self, slot: u32) -> Option<Range<usize>> {
        let start = usize::try_from(self.offset(slot)?).ok()?;
        let end = start.checked_add(usize::try_from(self.descriptor_size).ok()?)?;
        Some(start..end)
    }

    pub fn bytes<'a>(&self, heap: &'a mut [u8], slot: u32) -> Option<&'a mut [u8]> {
        heap.get_mut(self.range(slot)?)
    }

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

    pub const fn end(&self) -> u64 {
        (self.first_index as u64 + self.count as u64).saturating_mul(self.stride)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DescriptorHeapLayout {
    pub size: u64,
    pub alignment: u64,
    pub reserved_offset: u64,
    pub reserved_size: u64,
}

impl DescriptorHeapLayout {
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

    pub fn buffers(&mut self, count: u32) -> Result<DescriptorSlots, HeapLayoutError> {
        self.plan.run(
            count,
            self.buffer_size,
            self.buffer_size,
            self.buffer_alignment,
        )
    }

    pub fn images(&mut self, count: u32) -> Result<DescriptorSlots, HeapLayoutError> {
        self.plan.run(
            count,
            self.image_size,
            self.image_size,
            self.image_alignment,
        )
    }

    pub fn uniform(&mut self, count: u32) -> Result<DescriptorSlots, HeapLayoutError> {
        let alignment = lcm(self.buffer_alignment, self.image_alignment);
        let size = self.buffer_size.max(self.image_size);
        let stride = size
            .checked_next_multiple_of(alignment)
            .ok_or(HeapLayoutError::Overflow)?;
        self.plan.run(count, stride, size, alignment)
    }

    pub fn finish(self) -> Result<DescriptorHeapLayout, HeapLayoutError> {
        self.plan.finish()
    }
}

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
                reserved_alignment: properties.sampler_descriptor_alignment,
                reserved_size: properties.min_sampler_heap_reserved_range,
                heap_alignment: properties.sampler_heap_alignment,
                max_size: properties.max_sampler_heap_size,
            },
            sampler_size: properties.sampler_descriptor_size,
            sampler_alignment: properties.sampler_descriptor_alignment,
        }
    }

    pub fn samplers(&mut self, count: u32) -> Result<DescriptorSlots, HeapLayoutError> {
        self.plan.run(
            count,
            self.sampler_size,
            self.sampler_size,
            self.sampler_alignment,
        )
    }

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
