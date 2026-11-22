use vulkan::vk;

/// The memory property flags an allocation's memory type must have and should have.
///
/// [`Allocator::allocate`](crate::Allocator::allocate) considers only memory types with
/// every required flag. Among them it takes the lowest-indexed one with every preferred
/// flag, or else the one with the most preferred flags.
///
/// Memory types with `DEVICE_COHERENT_AMD` or `DEVICE_UNCACHED_AMD` are considered only
/// when the location names one of these two flags as required or preferred, which only
/// [`MemoryLocation::Custom`] can do: allocating from such a type is valid only if the
/// device was created with the `deviceCoherentMemory` feature.
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

const NEEDS_DEVICE_COHERENT_MEMORY: u32 = vk::MemoryPropertyFlags::DEVICE_COHERENT_AMD.as_raw()
    | vk::MemoryPropertyFlags::DEVICE_UNCACHED_AMD.as_raw();

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
        if flags.as_raw() & NEEDS_DEVICE_COHERENT_MEMORY != 0
            && (required.as_raw() | preferred.as_raw()) & NEEDS_DEVICE_COHERENT_MEMORY == 0
        {
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
) -> u64 {
    let mut alignment = requested_alignment.max(1);
    if flags.contains(vk::MemoryPropertyFlags::HOST_VISIBLE)
        && !flags.contains(vk::MemoryPropertyFlags::HOST_COHERENT)
    {
        alignment = alignment.max(non_coherent_atom_size.max(1));
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
    fn device_coherent_memory_types_are_chosen_only_when_the_location_names_them() {
        let amd = [
            vk::MemoryPropertyFlags::DEVICE_COHERENT_AMD,
            vk::MemoryPropertyFlags::DEVICE_UNCACHED_AMD
                | vk::MemoryPropertyFlags::DEVICE_COHERENT_AMD,
        ];
        let plain = [
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_CACHED,
            vk::MemoryPropertyFlags::DEVICE_LOCAL
                | vk::MemoryPropertyFlags::HOST_VISIBLE
                | vk::MemoryPropertyFlags::HOST_COHERENT,
        ];
        let mut kinds = plain.to_vec();
        for flags in plain {
            kinds.extend(amd.map(|amd| flags | amd));
        }
        let locations = [
            MemoryLocation::GpuOnly,
            MemoryLocation::CpuToGpu,
            MemoryLocation::GpuToCpu,
            MemoryLocation::custom(
                vk::MemoryPropertyFlags::empty(),
                vk::MemoryPropertyFlags::empty(),
            ),
            MemoryLocation::custom(
                vk::MemoryPropertyFlags::HOST_VISIBLE,
                vk::MemoryPropertyFlags::DEVICE_COHERENT_AMD,
            ),
            MemoryLocation::custom(
                vk::MemoryPropertyFlags::HOST_VISIBLE
                    | vk::MemoryPropertyFlags::DEVICE_UNCACHED_AMD,
                vk::MemoryPropertyFlags::empty(),
            ),
        ];
        let amd_flags = vk::MemoryPropertyFlags::DEVICE_COHERENT_AMD.as_raw()
            | vk::MemoryPropertyFlags::DEVICE_UNCACHED_AMD.as_raw();
        let is_amd = |flags: vk::MemoryPropertyFlags| flags.as_raw() & amd_flags != 0;
        let documented_choice = |flags: &[vk::MemoryPropertyFlags],
                                 memory_type_bits: u32,
                                 location: MemoryLocation,
                                 consider_unnamed_amd_types: bool| {
            let (required, preferred) = (location.required_flags(), location.preferred_flags());
            let named = is_amd(required) || is_amd(preferred);
            let candidates = (0..flags.len())
                .filter(|&index| memory_type_bits & (1 << index) != 0)
                .filter(|&index| flags[index].contains(required))
                .filter(|&index| consider_unnamed_amd_types || named || !is_amd(flags[index]))
                .collect::<Vec<_>>();
            let score = |index: usize| (flags[index].as_raw() & preferred.as_raw()).count_ones();
            candidates
                .iter()
                .copied()
                .find(|&index| flags[index].contains(preferred))
                .or_else(|| {
                    let best = candidates.iter().copied().map(score).max()?;
                    candidates
                        .iter()
                        .copied()
                        .find(|&index| score(index) == best)
                })
                .map(|index| index as u32)
        };

        let mut seed = 0x853c_49e6_748f_ea9b_u64;
        let mut next = move || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed
        };
        let mut left_out = 0;
        let mut taken = 0;
        for _ in 0..4000 {
            let flags = (0..1 + next() % 8)
                .map(|_| kinds[(next() % kinds.len() as u64) as usize])
                .collect::<Vec<_>>();
            let properties = memory_properties(&flags);
            let memory_type_bits = next() as u32 & ((1 << flags.len()) - 1);
            for location in locations {
                let chosen = find_memory_type_index(
                    &properties,
                    memory_type_bits,
                    location.required_flags(),
                    location.preferred_flags(),
                );
                assert_eq!(
                    chosen,
                    documented_choice(&flags, memory_type_bits, location, false),
                    "types {flags:?}, allowed {memory_type_bits:#b}, {location:?}"
                );
                if chosen.is_some_and(|index| is_amd(flags[index as usize])) {
                    taken += 1;
                }
                if chosen != documented_choice(&flags, memory_type_bits, location, true) {
                    left_out += 1;
                }
            }
        }
        assert!(left_out > 100 && taken > 100);
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
    fn cpu_to_gpu_chooses_coherent_memory_whenever_an_allowed_type_is_coherent() {
        let visible = vk::MemoryPropertyFlags::HOST_VISIBLE;
        let coherent = vk::MemoryPropertyFlags::HOST_COHERENT;
        let cached = vk::MemoryPropertyFlags::HOST_CACHED;
        let local = vk::MemoryPropertyFlags::DEVICE_LOCAL;
        let kinds = [
            local,
            visible,
            visible | cached,
            visible | coherent,
            visible | coherent | cached,
            local | visible,
            local | visible | cached,
            local | visible | coherent,
        ];
        let mut checked = 0;
        for combination in 0..kinds.len().pow(4) {
            let flags = (0..4)
                .map(|position| kinds[combination / kinds.len().pow(position) % kinds.len()])
                .collect::<Vec<_>>();
            let properties = memory_properties(&flags);
            for memory_type_bits in 1..16u32 {
                let allowed = |index: &usize| memory_type_bits & (1 << index) != 0;
                if !(0..flags.len())
                    .filter(allowed)
                    .any(|index| flags[index].contains(visible | coherent))
                {
                    continue;
                }
                let index = find_memory_type_index(
                    &properties,
                    memory_type_bits,
                    MemoryLocation::CpuToGpu.required_flags(),
                    MemoryLocation::CpuToGpu.preferred_flags(),
                )
                .expect("a host-visible type is allowed");
                assert!(
                    flags[index as usize].contains(visible | coherent),
                    "types {flags:?}, allowed {memory_type_bits:#06b}: chose type {index}"
                );
                checked += 1;
            }
        }
        assert!(checked > 0);
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
        assert_eq!(effective_alignment(non_coherent, 16, 256), 256);

        let coherent =
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT;
        assert_eq!(effective_alignment(coherent, 16, 256), 16);

        let device_local = vk::MemoryPropertyFlags::DEVICE_LOCAL;
        assert_eq!(effective_alignment(device_local, 16, 256), 16);

        let mut allocator = TlsfSuballocator::new(1024);
        let alignment = effective_alignment(non_coherent, 1, 256);
        let a = allocator.allocate(64, alignment).unwrap();
        let b = allocator.allocate(64, alignment).unwrap();
        assert_eq!(a.offset, 0);
        assert_eq!(b.offset, 256);
        check_invariants(&allocator);
    }
}
