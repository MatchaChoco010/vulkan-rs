# vulkan-alloc

A TLSF suballocator for `VkDeviceMemory`, built on the `vulkan` crate.

```rust
let mut allocator = Allocator::new(AllocatorCreateDesc::new(
    &device,
    instance.get_physical_device_memory_properties(physical_device),
    instance.get_physical_device_properties(physical_device),
));

let allocation = allocator.allocate(AllocationCreateDesc {
    name: Some("uniforms"),
    requirements: device.get_buffer_memory_requirements(buffer),
    location: MemoryLocation::CpuToGpu,
    linear: true,
    memory_allocate_flags: vk::MemoryAllocateFlags::empty(),
    export_handle_types: vk::ExternalMemoryHandleTypeFlags::empty(),
    allocation_scheme: AllocationScheme::GpuAllocatorManaged,
})?;
device.bind_buffer_memory(buffer, allocation.memory(), allocation.offset())?;

{
    // `map` returns a guard. Dropping it unmaps; do not call `unmap_memory` for it.
    let mut mapped = allocator.map(&allocation)?;
    mapped.byte_range_mut(0, 4).copy_from_slice(&[1, 2, 3, 4]);
}

device.destroy_buffer(buffer, None);
allocator.free(allocation)?;
```

## Things to know

- **Mapping has two forms that must not be mixed.** `map` returns a `MappedAllocation` guard that unmaps on drop. `map_memory` returns a raw pointer and needs exactly one `unmap_memory`.
- **Order of destruction is the caller's job.** Resources first, then their allocations, then the allocator, then the device. The allocator holds a copy of the `Device` wrapper, but a `Device` does not own the `VkDevice`.
- **Leaks are reported, not fatal.** Dropping an allocator with live allocations logs them through the `log` crate, with their names, and leaves their memory to the device. `live_allocations()` lists them; `free_all()` releases everything explicitly.
- **Blocks are shared only between like allocations**: same memory type, same linear / optimal class, same `memory_allocate_flags`.
- **Dedicated and exportable allocations get their own `VkDeviceMemory`.** Set `export_handle_types` for memory that will be exported, for example as a dma-buf.
- **Imported memory does not go through the allocator.** Allocate it with `vkAllocateMemory` directly; see `examples/dmabuf-roundtrip`.
- The allocator is `Send` and takes `&mut self`; wrap it in a `Mutex` to share it.

The crate documentation (`cargo doc -p vulkan-alloc --open`) has the details.
