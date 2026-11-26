use vulkan::vk;

/// The memory property flags an allocation's memory type must have, should have and
/// should not have.
///
/// [`Allocator::allocate`](crate::Allocator::allocate) considers only memory types with
/// every required flag. Among them it takes the one that lacks the fewest preferred flags
/// and has the fewest avoided flags, both counted together, and the lowest-indexed one of
/// those on a tie. So the lowest-indexed type with every preferred flag and no avoided
/// flag is taken whenever there is one. Only [`MemoryLocation::CpuOnly`] avoids a flag,
/// `DEVICE_LOCAL`. If the chosen memory type is out of memory, it takes the next one by
/// the same rule among the rest, so the memory of an allocation always has the required
/// flags, but can lack some of the preferred ones, or have an avoided one, even when
/// another allowed type would rank higher.
/// [`Allocation::memory_properties`](crate::Allocation::memory_properties) gives the flags
/// of the type that was chosen.
///
/// Which location to use depends on who accesses the memory:
///
/// - [`MemoryLocation::GpuOnly`]: only the device, such as images, vertex buffers and
///   the destination of an upload.
/// - [`MemoryLocation::CpuToGpu`]: the host writes and the device reads the memory
///   directly, such as uniform buffers and vertex buffers written every frame.
/// - [`MemoryLocation::CpuOnly`]: the host writes, and the device only copies from it,
///   such as staging buffers for uploads.
/// - [`MemoryLocation::GpuToCpu`]: the device writes and the host reads, such as
///   readback buffers.
///
/// [`MemoryLocation::CpuToGpu`], [`MemoryLocation::CpuOnly`] and
/// [`MemoryLocation::GpuToCpu`] require `HOST_VISIBLE` and `HOST_COHERENT`, so their
/// memory is always host-coherent: mapped memory can be written and read without
/// [`Allocator::flush_allocation`](crate::Allocator::flush_allocation) or
/// [`Allocator::invalidate_allocation`](crate::Allocator::invalidate_allocation), which
/// make no Vulkan call for such memory. If no memory type allowed by the resource's
/// `memory_type_bits` is host-visible and host-coherent,
/// [`Allocator::allocate`](crate::Allocator::allocate) fails with
/// [`AllocationError::NoCompatibleMemoryType`](crate::AllocationError::NoCompatibleMemoryType)
/// instead of taking memory that is not coherent. The Vulkan specification, describing
/// `VkMemoryRequirements::memoryTypeBits`, guarantees that the requirements of a buffer
/// created without `VK_BUFFER_CREATE_SPARSE_BINDING_BIT` and
/// `VK_BUFFER_CREATE_PROTECTED_BIT`, of a linear image created without
/// `VK_IMAGE_CREATE_PROTECTED_BIT`, and of a tensor (`VK_ARM_tensors`) created without
/// `VK_TENSOR_CREATE_PROTECTED_BIT_ARM`, allow a memory type with both `HOST_VISIBLE`
/// and `HOST_COHERENT`: "mappable coherent memory can always be attached to these
/// objects". So for those resources the error does not happen, unless every such
/// allowed type is one of the types left out below. For other resources, such as
/// images with optimal tiling, sparse buffers and protected resources, it can.
///
/// Memory that is `HOST_CACHED` but not `HOST_COHERENT` can be faster for the host to
/// read. To use it, give
/// `MemoryLocation::Custom { required: HOST_VISIBLE, preferred: HOST_VISIBLE | HOST_CACHED }`,
/// invalidate before reading and flush after writing. Both do nothing if the chosen type
/// is `HOST_COHERENT`, so they need not be skipped for it.
///
/// Two kinds of memory types are considered only when the location names their flag as
/// required or preferred, which only [`MemoryLocation::Custom`] can do:
///
/// - types with `DEVICE_COHERENT_AMD` or `DEVICE_UNCACHED_AMD`, when the location names
///   one of these two flags. Allocating from such a type is valid only if the device was
///   created with the `deviceCoherentMemory` feature.
/// - types with `PROTECTED`, when the location names `PROTECTED`. Allocating from such a
///   type is valid only if the device was created with the `protectedMemory` feature,
///   and protected resources, and only those, may be bound to it. Require the flag for
///   a protected resource, so that no other type can be chosen.
///
/// Memory types of a heap with `VK_MEMORY_HEAP_TILE_MEMORY_BIT_QCOM` are never chosen.
/// Only heaps below `memory_heap_count` count as such heaps.
/// Tile memory has its own requirements and is allocated with `vkAllocateMemory`
/// directly.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MemoryLocation {
    /// Memory for resources only the device accesses: requires and prefers
    /// `DEVICE_LOCAL`. The chosen type may also be host-visible, but nothing guarantees
    /// that, and a host-visible type chosen here need not be `HOST_COHERENT`.
    GpuOnly,
    /// Memory the host writes and the device reads directly, such as uniform buffers and
    /// vertex buffers that change every frame: requires `HOST_VISIBLE | HOST_COHERENT`
    /// and prefers `HOST_VISIBLE | HOST_COHERENT | DEVICE_LOCAL`. So among the allowed
    /// types with the required flags, the lowest-indexed one that is also
    /// `DEVICE_LOCAL` is taken, or the lowest-indexed one if none is. Host-visible
    /// device-local memory, such as the memory a discrete GPU exposes through its PCIe
    /// BAR or the memory of an integrated GPU, is faster for the device to read. When the
    /// chosen type is out of memory, the allocation moves to the next host-visible
    /// coherent type by the same rule, which is usually host memory (see
    /// [`Allocator::allocate`](crate::Allocator::allocate)). Host writes need no flush.
    ///
    /// On a logical device that represents more than one physical device, device-local
    /// heaps usually have `VK_MEMORY_HEAP_MULTI_INSTANCE_BIT`, and memory of such a heap
    /// must not be mapped (see the [crate documentation](crate#mapping)).
    CpuToGpu,
    /// Memory the device writes and the host reads back: requires
    /// `HOST_VISIBLE | HOST_COHERENT` and prefers
    /// `HOST_VISIBLE | HOST_COHERENT | HOST_CACHED`, so among the allowed types with the
    /// required flags, the lowest-indexed one that is also `HOST_CACHED` is taken, or the
    /// lowest-indexed one if none is. Host reads need no invalidate.
    GpuToCpu,
    /// Memory the host writes and the device only copies from, such as staging buffers
    /// for uploads: requires and prefers `HOST_VISIBLE | HOST_COHERENT` and avoids
    /// `DEVICE_LOCAL`. So among the allowed types with the required flags, the
    /// lowest-indexed one without `DEVICE_LOCAL` is taken, which keeps staging buffers out
    /// of a small host-visible device-local heap, or the lowest-indexed one if all of them
    /// are `DEVICE_LOCAL`, as on many integrated GPUs. When the memory without
    /// `DEVICE_LOCAL` is out of memory, the allocation moves to a device-local type (see
    /// [`Allocator::allocate`](crate::Allocator::allocate)). Host writes need no flush.
    ///
    /// For memory that the device writes and the host reads back, use
    /// [`MemoryLocation::GpuToCpu`], which prefers `HOST_CACHED` memory, faster for the
    /// host to read.
    CpuOnly,
    /// Explicit flags.
    ///
    /// The chosen type is certain to be `HOST_COHERENT` only if `required` contains that
    /// flag; otherwise flush after mapped writes and invalidate before mapped reads.
    /// [`Allocator::flush_allocation`](crate::Allocator::flush_allocation) and
    /// [`Allocator::invalidate_allocation`](crate::Allocator::invalidate_allocation) do
    /// nothing for a type that is `HOST_COHERENT`, so they can be called whichever type is
    /// chosen.
    Custom {
        /// Flags the memory type must have.
        required: vk::MemoryPropertyFlags,
        /// Flags the memory type should have. Types are ranked by how many of these
        /// flags they lack; they do not have to include `required`. A custom location
        /// avoids no flag.
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
            Self::CpuToGpu | Self::GpuToCpu | Self::CpuOnly => {
                vk::MemoryPropertyFlags::HOST_VISIBLE.union(vk::MemoryPropertyFlags::HOST_COHERENT)
            }
            Self::Custom { required, .. } => required,
        }
    }

    pub(crate) fn avoided_flags(self) -> vk::MemoryPropertyFlags {
        match self {
            Self::CpuOnly => vk::MemoryPropertyFlags::DEVICE_LOCAL,
            Self::GpuOnly | Self::CpuToGpu | Self::GpuToCpu | Self::Custom { .. } => {
                vk::MemoryPropertyFlags::empty()
            }
        }
    }

    pub(crate) fn preferred_flags(self) -> vk::MemoryPropertyFlags {
        match self {
            Self::GpuOnly => vk::MemoryPropertyFlags::DEVICE_LOCAL,
            Self::CpuToGpu => vk::MemoryPropertyFlags::HOST_VISIBLE
                .union(vk::MemoryPropertyFlags::HOST_COHERENT)
                .union(vk::MemoryPropertyFlags::DEVICE_LOCAL),
            Self::CpuOnly => {
                vk::MemoryPropertyFlags::HOST_VISIBLE.union(vk::MemoryPropertyFlags::HOST_COHERENT)
            }
            Self::GpuToCpu => vk::MemoryPropertyFlags::HOST_VISIBLE
                .union(vk::MemoryPropertyFlags::HOST_COHERENT)
                .union(vk::MemoryPropertyFlags::HOST_CACHED),
            Self::Custom { preferred, .. } => preferred,
        }
    }
}

const FLAGS_A_LOCATION_HAS_TO_NAME: [vk::MemoryPropertyFlags; 2] = [
    vk::MemoryPropertyFlags::DEVICE_COHERENT_AMD
        .union(vk::MemoryPropertyFlags::DEVICE_UNCACHED_AMD),
    vk::MemoryPropertyFlags::PROTECTED,
];

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

pub(crate) fn memory_heap(
    properties: &vk::PhysicalDeviceMemoryProperties,
    heap_index: u32,
) -> Option<&vk::MemoryHeap> {
    if heap_index >= properties.memory_heap_count {
        return None;
    }
    properties.memory_heaps.get(heap_index as usize)
}

pub(crate) fn find_memory_type_index(
    properties: &vk::PhysicalDeviceMemoryProperties,
    memory_type_bits: u32,
    required: vk::MemoryPropertyFlags,
    preferred: vk::MemoryPropertyFlags,
    avoided: vk::MemoryPropertyFlags,
) -> Option<u32> {
    let mut best = None;
    let mut best_cost = u32::MAX;
    for index in 0..properties.memory_type_count {
        let type_bit = 1u32.checked_shl(index).unwrap_or(0);
        if memory_type_bits & type_bit == 0 {
            continue;
        }
        let memory_type = properties.memory_types[index as usize];
        let flags = memory_type.property_flags;
        if !flags.contains(required) {
            continue;
        }
        if memory_heap(properties, memory_type.heap_index)
            .is_some_and(|heap| heap.flags.contains(vk::MemoryHeapFlags::TILE_MEMORY_QCOM))
        {
            continue;
        }
        let named = required.union(preferred);
        if FLAGS_A_LOCATION_HAS_TO_NAME
            .iter()
            .any(|&group| flags.intersects(group) && !named.intersects(group))
        {
            continue;
        }
        let cost = (preferred.as_raw() & !flags.as_raw()).count_ones()
            + (avoided.as_raw() & flags.as_raw()).count_ones();
        if cost < best_cost {
            best = Some(index);
            best_cost = cost;
        }
        if cost == 0 {
            return best;
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
    fn a_type_is_left_out_exactly_when_its_heap_is_a_reported_tile_memory_heap() {
        let locations = [
            MemoryLocation::GpuOnly,
            MemoryLocation::CpuToGpu,
            MemoryLocation::GpuToCpu,
            MemoryLocation::CpuOnly,
            MemoryLocation::custom(
                vk::MemoryPropertyFlags::empty(),
                vk::MemoryPropertyFlags::empty(),
            ),
        ];
        let kinds = [
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            vk::MemoryPropertyFlags::DEVICE_LOCAL
                | vk::MemoryPropertyFlags::HOST_VISIBLE
                | vk::MemoryPropertyFlags::HOST_COHERENT,
        ];
        let heap_counts = (0..=vk::MAX_MEMORY_HEAPS as u32)
            .chain([17, 32, 1000, u32::MAX])
            .collect::<Vec<_>>();
        let mut seed = 0x6a09_e667_f3bc_c909_u64;
        let mut next = move || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed
        };
        let mut left_out_inside = 0;
        let mut kept_outside = 0;
        for _ in 0..6000 {
            let flags = (0..1 + next() % 8)
                .map(|_| kinds[(next() % kinds.len() as u64) as usize])
                .collect::<Vec<_>>();
            let mut properties = memory_properties(&flags);
            properties.memory_heap_count =
                heap_counts[(next() % heap_counts.len() as u64) as usize];
            for heap in &mut properties.memory_heaps {
                if next() % 2 == 0 {
                    heap.flags = vk::MemoryHeapFlags::TILE_MEMORY_QCOM;
                }
            }
            let reported_heaps = properties
                .memory_heap_count
                .min(vk::MAX_MEMORY_HEAPS as u32);
            let mut in_tile_memory = Vec::new();
            for memory_type in &mut properties.memory_types[..flags.len()] {
                memory_type.heap_index = match next() % 8 {
                    0 => u32::MAX,
                    _ => (next() % (vk::MAX_MEMORY_HEAPS as u64 + 4)) as u32,
                };
                let heap = memory_type.heap_index;
                let tile_flag = properties
                    .memory_heaps
                    .get(heap as usize)
                    .is_some_and(|heap| heap.flags.contains(vk::MemoryHeapFlags::TILE_MEMORY_QCOM));
                in_tile_memory.push(heap < reported_heaps && tile_flag);
                left_out_inside += usize::from(heap < reported_heaps && tile_flag);
                kept_outside += usize::from(heap >= reported_heaps && tile_flag);
            }
            let memory_type_bits = next() as u32 & ((1 << flags.len()) - 1);
            for location in locations {
                let (required, preferred, avoided) = (
                    location.required_flags(),
                    location.preferred_flags(),
                    location.avoided_flags(),
                );
                let candidates = (0..flags.len())
                    .filter(|&index| memory_type_bits & (1 << index) != 0)
                    .filter(|&index| flags[index].contains(required))
                    .filter(|&index| !in_tile_memory[index])
                    .collect::<Vec<_>>();
                let cost = |index: usize| {
                    (preferred.as_raw() & !flags[index].as_raw()).count_ones()
                        + (avoided.as_raw() & flags[index].as_raw()).count_ones()
                };
                let documented = candidates
                    .iter()
                    .copied()
                    .min_by_key(|&index| cost(index))
                    .map(|index| index as u32);
                assert_eq!(
                    find_memory_type_index(
                        &properties,
                        memory_type_bits,
                        required,
                        preferred,
                        avoided
                    ),
                    documented,
                    "heap count {}, heaps of the types {:?}, types in tile memory {in_tile_memory:?}, \
                     allowed {memory_type_bits:#b}, {location:?}: only a heap below the heap count \
                     with TILE_MEMORY_QCOM may leave a type out",
                    properties.memory_heap_count,
                    properties.memory_types[..flags.len()]
                        .iter()
                        .map(|memory_type| memory_type.heap_index)
                        .collect::<Vec<_>>()
                );
            }
        }
        assert!(left_out_inside > 1000 && kept_outside > 1000);
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
            vk::MemoryPropertyFlags::empty(),
        );
        assert_eq!(index, Some(31));
    }
}
