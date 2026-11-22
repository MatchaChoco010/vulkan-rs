# vulkan-rs

Pure Rust Vulkan bindings generated from Khronos Vulkan-Headers.

## Crates

- `crates/vulkan`: the bindings. `vk` mirrors the C API; `Entry`, `Instance` and
  `Device` load the function pointers and expose the commands as methods.
- `crates/vulkan-alloc`: a TLSF device-memory allocator for `vulkan`.
- `crates/vulkan-codegen`: the generator of `crates/vulkan/src/vk/generated` and
  `crates/vulkan/tests/generated`. `tools/generate-vulkan` runs it.
- `crates/vulkan-registry`: the input of `vulkan-codegen`: the Vulkan registry and the
  descriptions from the specification, dumped to JSON by `tools/generate-vulkan`.
- `crates/source-lint`: tests the comment rules under `crates/` and the versions named
  in the Markdown files.

## Versioning

Workspace crate versions use Rust semver before `+` and record the
Vulkan-Headers SDK tag after `+`, for example
`0.4.0+vulkan-sdk-1.4.363.0`.

The SDK tag identifies which Vulkan headers were used. The semver part
identifies compatibility of this Rust API. Updating Vulkan-Headers should bump
the semver part as well. Use a minor bump for compatible header/API additions
while this crate is pre-1.0, and use the normal Rust semver rules for breaking
changes. The build metadata must still match the exact Vulkan-Headers SDK tag
used for generation.

## Updating Vulkan Bindings

```sh
git submodule update --init

# Checks out the newest `vulkan-sdk-*` tag in third_party/vulkan and the
# matching Vulkan-Docs tag in third_party/vulkan-docs, sets the workspace
# version in Cargo.toml and the Markdown files to "<semver>+<SDK tag>" and runs
# ./tools/generate-vulkan.
./tools/update-vulkan-headers

cargo test --workspace --all-features

SDK_TAG="$(git -C third_party/vulkan describe --tags --exact-match --match 'vulkan-sdk-*')"
CRATE_VERSION="$(sed -n 's/^version = "\(.*\)+.*"$/\1/p' Cargo.toml)"

git add third_party/vulkan third_party/vulkan-docs crates examples tools Cargo.toml Cargo.lock README.md
git commit -m "Update Vulkan bindings to $SDK_TAG"
git tag "v${CRATE_VERSION}+${SDK_TAG}"
```

Use `./tools/generate-vulkan --allow-non-sdk-tag` for development snapshots from
non-SDK Vulkan-Headers tags.
