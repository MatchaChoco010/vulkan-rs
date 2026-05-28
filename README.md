# vulkan-rs

Pure Rust Vulkan bindings generated from Khronos Vulkan-Headers.

## Versioning

Workspace crate versions use Rust semver before `+` and record the
Vulkan-Headers SDK tag after `+`, for example
`0.2.0+vulkan-sdk-1.4.350.0`.

The SDK tag identifies which Vulkan headers were used. The semver part
identifies compatibility of this Rust API. Updating Vulkan-Headers should bump
the semver part as well. Use a minor bump for compatible header/API additions
while this crate is pre-1.0, and use the normal Rust semver rules for breaking
changes. The build metadata must still match the exact Vulkan-Headers SDK tag
used for generation.

## Updating Vulkan Bindings

```sh
git -C third_party/vulkan fetch --tags
SDK_TAG="$(git -C third_party/vulkan tag --list 'vulkan-sdk-*' --sort=-v:refname | head -n1)"
git -C third_party/vulkan checkout "$SDK_TAG"

./tools/generate-vulkan

cargo test --workspace --all-features

# Choose the new crate semver version.
CRATE_VERSION="0.2.0"

# Set package.version to "${CRATE_VERSION}+${SDK_TAG}" in each workspace crate.
${EDITOR:-vi} crates/vulkan/Cargo.toml
${EDITOR:-vi} crates/vulkan-alloc/Cargo.toml
${EDITOR:-vi} crates/vulkan-codegen/Cargo.toml
${EDITOR:-vi} crates/vulkan-registry/Cargo.toml

git add third_party/vulkan crates/vulkan crates/vulkan-alloc crates/vulkan-codegen crates/vulkan-registry examples tools Cargo.toml Cargo.lock README.md
git commit -m "Update Vulkan bindings to $SDK_TAG"
git tag "v${CRATE_VERSION}+${SDK_TAG}"
```

Use `./tools/generate-vulkan --allow-non-sdk-tag` for development snapshots from
non-SDK Vulkan-Headers tags.
