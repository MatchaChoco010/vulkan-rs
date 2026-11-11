# vulkan-rs

Pure Rust Vulkan bindings generated from Khronos [Vulkan-Headers](https://github.com/KhronosGroup/Vulkan-Headers).

- **One to one with the C API.** Types, constants and commands come out as the registry defines them.
- **Structs cannot outlive what they point to.** Code that would leave a dangling pointer in a create-info struct does not compile. There is no `build()` step that erases the lifetime.
- **Checked against the C headers.** A test compiles a generated C program against the Khronos headers and compares struct sizes, member offsets, bit-field positions and constant values with the Rust bindings.
- **Registry data the headers do not carry**: the texel layout of every format (`vk::Format::info`) and the dependencies of every extension (`vk::ExtensionInfo`).

Generated from `vulkan-sdk-1.4.363.0`.

| Crate | |
| --- | --- |
| [`vulkan`](crates/vulkan) | The bindings |
| [`vulkan-alloc`](crates/vulkan-alloc) | A TLSF suballocator for `VkDeviceMemory` |
| [`vulkan-registry`](crates/vulkan-registry) | The registry dump the generator reads |
| [`vulkan-codegen`](crates/vulkan-codegen) | The generator |

## Documentation

The design documents are written in Japanese.

- [docs/design.md](docs/design.md): layout of the repository, the generation pipeline, the API rules, the update procedure. （構成、生成の流れ、API の規則、更新の手順）
- [docs/audit.md](docs/audit.md): what was wrong before the 0.3 rewrite and how each problem was confirmed. （作り直しの前の監査結果）
- [docs/safety-layer.md](docs/safety-layer.md): which misuses of Vulkan can be prevented by types, by run-time checks, or not at all, and where a checking layer should live. （誤った使い方で落ちない層についての知見）
- [AGENTS.md](AGENTS.md): how to regenerate, verify and what not to do. （開発の手順）

The API documentation is rustdoc, in English: `cargo doc -p vulkan -p vulkan-alloc --open`.

## Getting started

The crates are not published on crates.io. Depend on them through git or a path.

```toml
[dependencies]
vulkan = { git = "https://github.com/MatchaChoco010/vulkan-rs", features = ["wayland"] }
vulkan-alloc = { git = "https://github.com/MatchaChoco010/vulkan-rs" }
```

Creating an instance and a device:

```rust
use vulkan::{Entry, vk};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // SAFETY: this example upholds the valid usage rules of the commands it calls.
    unsafe {
        let entry = Entry::load()?;

        let app = vk::ApplicationInfo::default()
            .application_name(c"example")
            .api_version(vk::API_VERSION_1_3);
        let instance =
            entry.create_instance(&vk::InstanceCreateInfo::default().application_info(&app), None)?;

        let physical_device = instance.enumerate_physical_devices()?[0];
        let properties = instance.get_physical_device_properties(physical_device);
        println!("{:?}", properties.device_name_as_c_str()?);

        // The structs cannot outlive `priorities`, `queues` and `vulkan13`.
        let priorities = [1.0];
        let queues = [vk::DeviceQueueCreateInfo::default()
            .queue_family_index(0)
            .queue_priorities(&priorities)];
        let mut vulkan13 = vk::PhysicalDeviceVulkan13Features::default()
            .dynamic_rendering(true)
            .synchronization2(true);
        let device = instance.create_device(
            physical_device,
            &vk::DeviceCreateInfo::default()
                .queue_create_infos(&queues)
                .push_next(&mut vulkan13),
            None,
        )?;
        let queue = device.get_device_queue(0, 0);
        println!("{queue:?}");

        device.destroy_device(None);
        instance.destroy_instance(None);
    }
    Ok(())
}
```

The shape of the API in a few lines:

- Every command is an `unsafe fn` on `Entry`, `Instance` or `Device`, in two forms: `device.create_buffer(&info, None)` with slices, references and `vk::Result`, and `device.create_buffer_raw(..)` with the C signature.
- All commands of a level are loaded when the `Instance` or `Device` is created. There are no per-extension loader types. Calling a command the loader did not provide panics; `device.fp().create_swapchain_khr.is_some()` tells in advance.
- Setters borrow: slices for pointer + count, `&CStr` for strings, `&[vk::CStrPtr]` for arrays of strings, `push_next(&mut ext)` for `pNext`.
- Commands with more than one success code return the code in `Ok`: `device.wait_for_fences(..)` is `Ok(VkResult::SUCCESS)` or `Ok(VkResult::TIMEOUT)`.
- Enums, flags and handles print their names with `Debug`.

Where to find a working example of each thing:

| | |
| --- | --- |
| Instance and device creation, validation layer, `pNext` queries | [`crates/vulkan/tests/device.rs`](crates/vulkan/tests/device.rs), `Gpu::create` |
| Drawing with `VK_EXT_descriptor_heap` (no descriptor sets) | `crates/vulkan/tests/device.rs`, `descriptor_heap_draws_through_resource_and_sampler_heaps`; [`examples/descriptor-heap-14`](examples/descriptor-heap-14/src/main.rs) |
| Wayland surface and swapchain, color space, present mode, HDR metadata | [`examples/wayland-dynamic-rendering`](examples/wayland-dynamic-rendering/src/main.rs) |
| dma-buf export and import with DRM format modifiers | [`examples/dmabuf-roundtrip`](examples/dmabuf-roundtrip/src/main.rs), `export_image` and `import_image` |
| DRM device nodes (`VK_EXT_physical_device_drm`) | the same example, `print_drm_nodes` |
| Headless rendering and readback | [`examples/headless-triangle`](examples/headless-triangle/src/main.rs) |
| Dynamic rendering, synchronization2, timeline semaphores | `crates/vulkan/tests/device.rs` |
| Passing fences and semaphores as sync files | `crates/vulkan/tests/device.rs` |
| Y'CbCr conversion | `crates/vulkan/tests/device.rs` |
| Format layouts and extension dependencies | [`crates/vulkan/tests/api.rs`](crates/vulkan/tests/api.rs), `formats_know_their_texel_layout`, `extensions_know_their_dependencies` |
| Memory allocation, exportable memory, leak reporting | [`crates/vulkan-alloc`](crates/vulkan-alloc), [`crates/vulkan-alloc/tests/device.rs`](crates/vulkan-alloc/tests/device.rs) |

## Examples

```sh
cargo run -p vulkan-headless-triangle
cargo run -p vulkan-dmabuf-roundtrip
cargo run -p vulkan-descriptor-heap-14
cargo run -p vulkan-wayland-dynamic-rendering
cargo run -p vulkan-wayland-triangle
cargo run -p vulkan-compute-buffer          # uses descriptor sets
cargo run -p vulkan-mesh-shader             # uses descriptor sets
cargo run -p vulkan-ray-tracing-pipeline    # uses descriptor sets
```

Headless examples write an image to `target/example-output/`. All except `dmabuf-roundtrip` require the validation layer (`VK_LAYER_KHRONOS_validation`) to be installed.

`wayland-dynamic-rendering` takes its swapchain settings from the environment:

| Variable | Values |
| --- | --- |
| `VULKAN_EXAMPLE_PRESENT_MODE` | `fifo` (default), `fifo_relaxed`, `immediate` (tearing), `mailbox` |
| `VULKAN_EXAMPLE_COLOR_SPACE` | `srgb` (default), `hdr10`, `scrgb` |
| `VULKAN_EXAMPLE_FRAMES` | exit after this many frames |

To run on lavapipe, the software implementation, select its ICD:

```sh
VK_DRIVER_FILES=/usr/share/vulkan/icd.d/lvp_icd.x86_64.json cargo run -p vulkan-headless-triangle
```

## Versioning

The workspace version is `<semver>+<Vulkan-Headers SDK tag>`, for example `0.3.0+vulkan-sdk-1.4.363.0`. It is set once in the root `Cargo.toml` and inherited by every crate.

- The part after `+` is the SDK tag the bindings were generated from.
- Before 1.0, the minor version is bumped for a Vulkan-Headers update and for an incompatible change of the Rust API. A header update is not guaranteed to be compatible: provisional extensions get renamed and removed.
- Compatible fixes bump the patch version.

## Updating Vulkan-Headers

```sh
git submodule update --init
./tools/update-vulkan-headers
cargo fmt --check
cargo clippy --workspace --all-features --all-targets
cargo test --workspace --all-features
```

`tools/update-vulkan-headers` moves the submodule to the newest SDK tag, bumps the version and regenerates. A GitHub Actions workflow (`.github/workflows/update-vulkan-headers.yml`) does this weekly and opens a pull request.

To generate from a commit that is not an SDK tag, move the submodule by hand and run `./tools/generate-vulkan --allow-non-sdk-tag`.

## License

[Apache-2.0](LICENSE)
