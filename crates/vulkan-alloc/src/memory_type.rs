use vulkan::vk;

use crate::allocation::AllocationResourceClass;

/// The memory property flags an allocation's memory type must have and should have.
///
/// [`Allocator::allocate`](crate::Allocator::allocate) considers only memory types with
/// every required flag. Among them it takes the lowest-indexed one with every preferred
/// flag, or else the one with the most preferred flags.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MemoryLocation {
    /// Memory for resources only the device accesses: requires and prefers
    /// `DEVICE_LOCAL`. The chosen type may also be host-visible, but nothing guarantees
    /// that.
    GpuOnly,
    /// Memory the host writes and the device reads, such as staging and uniform
    /// buffers: requires `HOST_VISIBLE` and prefers `HOST_VISIBLE | HOST_COHERENT`.
    CpuToGpu,
    /// Memory the device writes and the host reads back: requires `HOST_VISIBLE` and
    /// prefers `HOST_VISIBLE | HOST_CACHED`. The chosen type need not be
    /// `HOST_COHERENT`; check
    /// [`Allocator::allocation_memory_flags`](crate::Allocator::allocation_memory_flags)
    /// to know whether reads need an invalidate first.
    GpuToCpu,
    /// Explicit flags.
    Custom {
        /// Flags the memory type must have.
        required: vk::MemoryPropertyFlags,
        /// Flags the memory type should have. Types are ranked by how many of these
        /// flags they have; they do not have to include `required`.
        preferred: vk::MemoryPropertyFlags,
    },
}

impl MemoryLocation {
    /// The same as [`MemoryLocation::Custom`] with these fields.
    ///
    /// ```
    /// use vulkan::vk;
    /// use vulkan_alloc::MemoryLocation;
    ///
    /// let location = MemoryLocation::custom(
    ///     vk::MemoryPropertyFlags::HOST_VISIBLE,
    ///     vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_CACHED,
    /// );
    /// assert_eq!(
    ///     location,
    ///     MemoryLocation::Custom {
    ///         required: vk::MemoryPropertyFlags::HOST_VISIBLE,
    ///         preferred: vk::MemoryPropertyFlags::HOST_VISIBLE
    ///             | vk::MemoryPropertyFlags::HOST_CACHED,
    ///     }
    /// );
    /// ```
    pub const fn custom(
        required: vk::MemoryPropertyFlags,
        preferred: vk::MemoryPropertyFlags,
    ) -> Self {
        Self::Custom {
            required,
            preferred,
        }
    }

    pub(crate) fn required_flags(self) -> vk::MemoryPropertyFlags {
        match self {
            Self::GpuOnly => vk::MemoryPropertyFlags::DEVICE_LOCAL,
            Self::CpuToGpu | Self::GpuToCpu => vk::MemoryPropertyFlags::HOST_VISIBLE,
            Self::Custom { required, .. } => required,
        }
    }

    pub(crate) fn preferred_flags(self) -> vk::MemoryPropertyFlags {
        match self {
            Self::GpuOnly => vk::MemoryPropertyFlags::DEVICE_LOCAL,
            Self::CpuToGpu => {
                vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT
            }
            Self::GpuToCpu => {
                vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_CACHED
            }
            Self::Custom { preferred, .. } => preferred,
        }
    }
}

pub(crate) fn memory_type_flags(
    properties: &vk::PhysicalDeviceMemoryProperties,
    memory_type_index: u32,
) -> Option<vk::MemoryPropertyFlags> {
    if memory_type_index >= properties.memory_type_count {
        return None;
    }
    properties
        .memory_types
        .get(memory_type_index as usize)
        .map(|memory_type| memory_type.property_flags)
}

pub(crate) fn find_memory_type_index(
    properties: &vk::PhysicalDeviceMemoryProperties,
    memory_type_bits: u32,
    required: vk::MemoryPropertyFlags,
    preferred: vk::MemoryPropertyFlags,
) -> Option<u32> {
    let mut best = None;
    let mut best_score = 0;
    for index in 0..properties.memory_type_count {
        let type_bit = 1u32.checked_shl(index).unwrap_or(0);
        if memory_type_bits & type_bit == 0 {
            continue;
        }
        let flags = properties.memory_types[index as usize].property_flags;
        if !flags.contains(required) {
            continue;
        }
        let score = (flags.as_raw() & preferred.as_raw()).count_ones();
        if best.is_none() || score > best_score {
            best = Some(index);
            best_score = score;
        }
        if flags.contains(preferred) {
            return Some(index);
        }
    }
    best
}

pub(crate) fn effective_alignment(
    flags: vk::MemoryPropertyFlags,
    requested_alignment: u64,
    non_coherent_atom_size: u64,
    buffer_image_granularity: u64,
    resource_class: AllocationResourceClass,
) -> u64 {
    let mut alignment = requested_alignment.max(1);
    if flags.contains(vk::MemoryPropertyFlags::HOST_VISIBLE)
        && !flags.contains(vk::MemoryPropertyFlags::HOST_COHERENT)
    {
        alignment = alignment.max(non_coherent_atom_size.max(1));
    }
    if resource_class == AllocationResourceClass::Unknown {
        alignment = alignment.max(buffer_image_granularity.max(1));
    }
    alignment
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::suballocator::TlsfSuballocator;
    use crate::suballocator::tests::check_invariants;

    pub(crate) fn memory_properties(
        flags: &[vk::MemoryPropertyFlags],
    ) -> vk::PhysicalDeviceMemoryProperties {
        let mut properties = vk::PhysicalDeviceMemoryProperties {
            memory_type_count: flags.len() as u32,
            ..Default::default()
        };
        for (index, flags) in flags.iter().copied().enumerate() {
            properties.memory_types[index].property_flags = flags;
        }
        properties
    }

    #[test]
    fn memory_type_flags_rejects_indices_outside_the_reported_types() {
        let properties = memory_properties(&[
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
            vk::MemoryPropertyFlags::HOST_VISIBLE,
        ]);
        assert_eq!(
            memory_type_flags(&properties, 1),
            Some(vk::MemoryPropertyFlags::HOST_VISIBLE)
        );
        assert_eq!(memory_type_flags(&properties, 2), None);

        let overreported = vk::PhysicalDeviceMemoryProperties {
            memory_type_count: 40,
            ..properties
        };
        assert_eq!(
            memory_type_flags(&overreported, 31),
            Some(vk::MemoryPropertyFlags::empty())
        );
        assert_eq!(memory_type_flags(&overreported, 32), None);
        assert_eq!(memory_type_flags(&overreported, 39), None);
    }

    #[test]
    fn finds_preferred_memory_type_when_available() {
        let properties = memory_properties(&[
            vk::MemoryPropertyFlags::HOST_VISIBLE,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
        ]);

        let index = find_memory_type_index(
            &properties,
            0b111,
            vk::MemoryPropertyFlags::HOST_VISIBLE,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
        );
        assert_eq!(index, Some(1));
    }

    #[test]
    fn preferred_memory_type_must_be_allowed_by_type_bits() {
        let properties = memory_properties(&[
            vk::MemoryPropertyFlags::HOST_VISIBLE,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
        ]);

        let index = find_memory_type_index(
            &properties,
            0b001,
            vk::MemoryPropertyFlags::HOST_VISIBLE,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
        );
        assert_eq!(index, Some(0));
    }

    #[test]
    fn falls_back_to_required_memory_type() {
        let properties = memory_properties(&[
            vk::MemoryPropertyFlags::HOST_VISIBLE,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
        ]);

        let index = find_memory_type_index(
            &properties,
            0b01,
            vk::MemoryPropertyFlags::HOST_VISIBLE,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
        );
        assert_eq!(index, Some(0));
    }

    #[test]
    fn rejects_incompatible_memory_type_bits() {
        let properties = memory_properties(&[vk::MemoryPropertyFlags::HOST_VISIBLE]);

        let index = find_memory_type_index(
            &properties,
            0,
            vk::MemoryPropertyFlags::HOST_VISIBLE,
            vk::MemoryPropertyFlags::HOST_VISIBLE,
        );
        assert_eq!(index, None);
    }

    #[test]
    fn rejects_when_allowed_types_do_not_have_required_flags() {
        let properties = memory_properties(&[
            vk::MemoryPropertyFlags::HOST_VISIBLE,
            vk::MemoryPropertyFlags::HOST_COHERENT,
        ]);

        let index = find_memory_type_index(
            &properties,
            0b11,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
        );
        assert_eq!(index, None);
    }

    #[test]
    fn finds_high_memory_type_bit() {
        let mut properties = vk::PhysicalDeviceMemoryProperties {
            memory_type_count: 32,
            ..Default::default()
        };
        properties.memory_types[31].property_flags = vk::MemoryPropertyFlags::DEVICE_LOCAL;

        let index = find_memory_type_index(
            &properties,
            1 << 31,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
        );
        assert_eq!(index, Some(31));
    }

    #[test]
    fn gpu_to_cpu_prefers_cached_memory_over_uncached_coherent_memory() {
        let properties = memory_properties(&[
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_CACHED,
        ]);

        let index = find_memory_type_index(
            &properties,
            0b11,
            MemoryLocation::GpuToCpu.required_flags(),
            MemoryLocation::GpuToCpu.preferred_flags(),
        );
        assert_eq!(index, Some(1));
    }

    #[test]
    fn preferred_memory_type_score_counts_matching_flags() {
        let properties = memory_properties(&[
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_CACHED,
        ]);

        let preferred = vk::MemoryPropertyFlags::HOST_VISIBLE
            | vk::MemoryPropertyFlags::HOST_COHERENT
            | vk::MemoryPropertyFlags::HOST_CACHED;
        let index = find_memory_type_index(
            &properties,
            0b11,
            vk::MemoryPropertyFlags::HOST_VISIBLE,
            preferred,
        );
        assert_eq!(index, Some(0));
    }

    #[test]
    fn memory_location_flags_match_intended_usage() {
        assert_eq!(
            MemoryLocation::GpuOnly.required_flags(),
            vk::MemoryPropertyFlags::DEVICE_LOCAL
        );
        assert_eq!(
            MemoryLocation::CpuToGpu.required_flags(),
            vk::MemoryPropertyFlags::HOST_VISIBLE
        );
        assert!(
            MemoryLocation::CpuToGpu
                .preferred_flags()
                .contains(vk::MemoryPropertyFlags::HOST_COHERENT)
        );
        assert!(
            MemoryLocation::GpuToCpu
                .preferred_flags()
                .contains(vk::MemoryPropertyFlags::HOST_CACHED)
        );

        let custom = MemoryLocation::custom(
            vk::MemoryPropertyFlags::HOST_VISIBLE,
            vk::MemoryPropertyFlags::HOST_VISIBLE
                | vk::MemoryPropertyFlags::HOST_COHERENT
                | vk::MemoryPropertyFlags::HOST_CACHED,
        );
        assert_eq!(
            custom.required_flags(),
            vk::MemoryPropertyFlags::HOST_VISIBLE
        );
        assert!(
            custom
                .preferred_flags()
                .contains(vk::MemoryPropertyFlags::HOST_CACHED)
        );
    }

    #[test]
    fn non_coherent_host_visible_allocations_use_atom_alignment() {
        let non_coherent =
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_CACHED;
        assert_eq!(
            effective_alignment(non_coherent, 16, 256, 1, AllocationResourceClass::Unknown),
            256
        );

        let coherent =
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT;
        assert_eq!(
            effective_alignment(coherent, 16, 256, 1, AllocationResourceClass::Unknown),
            16
        );

        let device_local = vk::MemoryPropertyFlags::DEVICE_LOCAL;
        assert_eq!(
            effective_alignment(device_local, 16, 256, 512, AllocationResourceClass::Optimal),
            16
        );
        assert_eq!(
            effective_alignment(device_local, 16, 256, 512, AllocationResourceClass::Unknown),
            512
        );

        let mut allocator = TlsfSuballocator::new(1024);
        let alignment =
            effective_alignment(non_coherent, 1, 256, 1, AllocationResourceClass::Unknown);
        let a = allocator.allocate(64, alignment).unwrap();
        let b = allocator.allocate(64, alignment).unwrap();
        assert_eq!(a.offset, 0);
        assert_eq!(b.offset, 256);
        check_invariants(&allocator);
    }
}
