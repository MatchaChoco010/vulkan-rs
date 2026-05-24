# vulkan-rs

Pure Rust Vulkan bindings generated from Khronos Vulkan-Headers.

## Versioning

The `vulkan` crate version uses Rust semver before `+` and records the
Vulkan-Headers SDK tag after `+`, for example
`0.1.0+vulkan-sdk-1.4.350.0`.

The SDK tag identifies which Vulkan headers were used. The semver part
identifies compatibility of this Rust API. Updating Vulkan-Headers should bump
the semver part as well, because SDK releases normally add generated Rust API
surface such as new constants, structs, commands, or extension wrappers. Use a
minor bump for compatible header/API additions while this crate is pre-1.0, and
use the normal Rust semver rules for breaking changes. The build metadata must
still match the exact Vulkan-Headers SDK tag used for generation.

## Updating Vulkan Bindings

```sh
git -C third_party/vulkan fetch --tags
SDK_TAG="$(git -C third_party/vulkan tag --list 'vulkan-sdk-*' --sort=-v:refname | head -n1)"
git -C third_party/vulkan checkout "$SDK_TAG"

./tools/generate-vulkan

cargo test --workspace --all-features

git diff -- crates/vulkan/src/vk/generated
${EDITOR:-vi} crates/vulkan/Cargo.toml
git add third_party/vulkan crates/vulkan
git commit -m "Update Vulkan bindings to $SDK_TAG"
git tag "$SDK_TAG"
```

Use `./tools/generate-vulkan --allow-non-sdk-tag` for development snapshots from
non-SDK Vulkan-Headers tags.

`./tools/generate-vulkan` records the Vulkan-Headers tag and commit in generated
metadata. Update `crates/vulkan/Cargo.toml` manually because the Rust crate
semver communicates API compatibility separately from the Vulkan header version.
