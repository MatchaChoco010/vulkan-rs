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
git submodule update --init

# Checks out the newest `vulkan-sdk-*` tag in third_party/vulkan, sets the
# workspace version in Cargo.toml to "<semver>+<SDK tag>" and runs
# ./tools/generate-vulkan.
./tools/update-vulkan-headers

cargo test --workspace --all-features

SDK_TAG="$(git -C third_party/vulkan describe --tags --exact-match --match 'vulkan-sdk-*')"
CRATE_VERSION="$(sed -n 's/^version = "\(.*\)+.*"$/\1/p' Cargo.toml)"

git add third_party/vulkan crates examples tools Cargo.toml Cargo.lock README.md
git commit -m "Update Vulkan bindings to $SDK_TAG"
git tag "v${CRATE_VERSION}+${SDK_TAG}"
```

Use `./tools/generate-vulkan --allow-non-sdk-tag` for development snapshots from
non-SDK Vulkan-Headers tags.
