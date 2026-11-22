# vulkan-registry

The input of `vulkan-codegen`, which generates the `vulkan` crate from it.

- `vulkan-ir.json`: the parts of the Vulkan registry (`vk.xml` and `video.xml` of
  Vulkan-Headers) that the generator uses.
- `vulkan-docs.json`: the descriptions that the Vulkan Specification (Vulkan-Docs) gives
  for those names. They become the documentation of the generated items.

`tools/generate-vulkan` writes both files with Khronos' Python registry tools. The crate
embeds them and reads them into `Registry` and `Docs`.
