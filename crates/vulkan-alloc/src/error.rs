use vulkan::vk;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AllocationError {
    NoCompatibleMemoryType,
    OutOfMemory,
    Vulkan(vk::VkResult),
    InvalidAllocation,
}

impl std::fmt::Display for AllocationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoCompatibleMemoryType => write!(f, "no compatible Vulkan memory type"),
            Self::OutOfMemory => write!(f, "Vulkan allocator is out of memory"),
            Self::Vulkan(err) => write!(f, "{err}"),
            Self::InvalidAllocation => write!(f, "invalid or already freed allocation"),
        }
    }
}

impl std::error::Error for AllocationError {}

impl From<vk::VkResult> for AllocationError {
    fn from(value: vk::VkResult) -> Self {
        Self::Vulkan(value)
    }
}
