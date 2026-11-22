use std::fmt;

use vulkan::vk;

use crate::memory_type::MemoryLocation;

#[derive(Copy, Clone)]
pub struct AllocationCreateDesc<'a> {
    pub name: Option<&'a str>,
    pub requirements: vk::MemoryRequirements,
    pub location: MemoryLocation,
    pub linear: bool,
    pub memory_allocate_flags: vk::MemoryAllocateFlags,
    pub export_handle_types: vk::ExternalMemoryHandleTypeFlags,
    pub allocation_scheme: AllocationScheme,
}

impl fmt::Debug for AllocationCreateDesc<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AllocationCreateDesc")
            .field("name", &self.name)
            .field("requirements.size", &self.requirements.size)
            .field("requirements.alignment", &self.requirements.alignment)
            .field(
                "requirements.memory_type_bits",
                &self.requirements.memory_type_bits,
            )
            .field("location", &self.location)
            .field("linear", &self.linear)
            .field("memory_allocate_flags", &self.memory_allocate_flags)
            .field("export_handle_types", &self.export_handle_types)
            .field("allocation_scheme", &self.allocation_scheme)
            .finish()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AllocationScheme {
    GpuAllocatorManaged,
    DedicatedBuffer(vk::Buffer),
    DedicatedImage(vk::Image),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Allocation {
    pub(crate) memory_type_index: u32,
    pub(crate) block_index: usize,
    pub(crate) allocation_id: u64,
    pub(crate) resource_class: AllocationResourceClass,
    pub(crate) memory: vk::DeviceMemory,
    pub(crate) offset: u64,
    pub(crate) size: u64,
}

impl Allocation {
    pub fn memory(&self) -> vk::DeviceMemory {
        self.memory
    }

    pub fn offset(&self) -> u64 {
        self.offset
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn memory_type_index(&self) -> u32 {
        self.memory_type_index
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AllocationResourceClass {
    Unknown,
    Linear,
    Optimal,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_allocation_desc_can_request_dedicated_resources() {
        let requirements = vk::MemoryRequirements {
            size: 1024,
            alignment: 256,
            memory_type_bits: 1,
        };

        let buffer_desc = AllocationCreateDesc {
            name: Some("buffer"),
            requirements,
            location: MemoryLocation::GpuOnly,
            linear: true,
            memory_allocate_flags: vk::MemoryAllocateFlags::empty(),
            export_handle_types: vk::ExternalMemoryHandleTypeFlags::empty(),
            allocation_scheme: AllocationScheme::DedicatedBuffer(vk::Buffer(7)),
        };
        assert_eq!(
            buffer_desc.allocation_scheme,
            AllocationScheme::DedicatedBuffer(vk::Buffer(7))
        );
        assert!(buffer_desc.linear);

        let image_desc = AllocationCreateDesc {
            name: Some("image"),
            requirements,
            location: MemoryLocation::GpuOnly,
            linear: false,
            memory_allocate_flags: vk::MemoryAllocateFlags::empty(),
            export_handle_types: vk::ExternalMemoryHandleTypeFlags::empty(),
            allocation_scheme: AllocationScheme::DedicatedImage(vk::Image(9)),
        };
        assert_eq!(
            image_desc.allocation_scheme,
            AllocationScheme::DedicatedImage(vk::Image(9))
        );
        assert!(!image_desc.linear);
    }
}
