# vulkan

Pure Rust Vulkan bindings generated from Khronos Vulkan-Headers.

```rust
use vulkan::{Entry, vk};

unsafe {
    let entry = Entry::load()?;
    let app = vk::ApplicationInfo::default().api_version(vk::API_VERSION_1_3);
    let instance =
        entry.create_instance(&vk::InstanceCreateInfo::default().application_info(&app), None)?;
    for physical_device in instance.enumerate_physical_devices()? {
        let properties = instance.get_physical_device_properties(physical_device);
        println!("{:?}", properties.device_name_as_c_str()?);
    }
    instance.destroy_instance(None);
}
```

## Layout

- `vulkan::vk`: types, constants and function pointer tables, one to one with the C API. `VkBufferCreateInfo` is `vk::BufferCreateInfo`, `VK_FORMAT_R8G8B8A8_UNORM` is `vk::Format::R8G8B8A8_UNORM`.
- `vulkan::Entry`, `Instance`, `Device`: a handle and the function pointer table of its level, with every command as a method. They do not destroy anything on drop.

## Rules worth knowing

- **Lifetimes.** A struct that can hold a pointer has a lifetime, and its setters take references and slices with that lifetime. Writing a pointer field directly bypasses it.
- **`pNext`.** `root.push_next(&mut ext)` prepends `ext` and overwrites `ext.p_next`. Push structs one at a time. Only structs the registry lists for the root are accepted.
- **Commands.** `device.foo(..)` takes slices and references and returns `vk::Result`; `device.foo_raw(..)` is the C signature. Every command is `unsafe`.
- **Loading.** Creating an `Instance` or `Device` loads every command of that level. A command that was not provided is `None` in `fp()` and panics when called.
- **Names.** `type` members are `ty`. Constants that would start with a digit get an underscore: `vk::ImageType::_2D`.
- **Formats and extensions.** `vk::Format::info()` gives the texel layout of a format. `vk::ExtensionInfo::get(name)` gives the dependencies of an extension.

## Features

| Feature | |
| --- | --- |
| `std` (default) | `Entry::load`. Without it the crate is `no_std` + `alloc` |
| `debug` (default) | `Debug` for structs |
| `linked` | `Entry::linked`, linking the Vulkan loader at build time |
| `wayland`, `xcb`, `xlib`, `win32`, ... | platform types and commands |
| `beta` | provisional extensions |

`src/vk/generated/` and `tests/generated/` are written by `tools/generate-vulkan`. See the [repository README](../../README.md) for examples and [docs/design.md](../../docs/design.md) (Japanese) for the full set of rules.
