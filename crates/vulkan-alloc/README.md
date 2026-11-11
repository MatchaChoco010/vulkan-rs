# vulkan-alloc

`vulkan` crate の上に作った、`VkDeviceMemory` の suballocator（TLSF）です。

- 同じ種類のメモリ（memory type、linear / optimal、allocate flags）の確保を、1 つの `VkDeviceMemory` から切り出します。
- dedicated な確保と、export できるメモリ（`export_handle_types`）は、専用の `VkDeviceMemory` にします。
- import するメモリ（dma-buf など）は扱いません。`vkAllocateMemory` を直接呼びます。同じ device で併用できます。
- 確保が残ったまま `Allocator` を drop すると panic します。
