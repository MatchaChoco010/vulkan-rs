# vulkan-codegen

The generator of the `vulkan` crate. Run it through `tools/generate-vulkan`.

| Module | |
| --- | --- |
| `naming` | C identifiers to Rust identifiers |
| `model` | type lookup, C to Rust type mapping, lifetime analysis, dispatch levels, dependency expressions |
| `emit::basic` | constants, handles, enums, flags, function pointer types, extension names |
| `emit::structs` | structs and unions, setters, bit-fields, `push_next` |
| `emit::commands` | function pointer tables and command wrappers |
| `emit::tables` | format layouts and extension dependencies |
| `emit::layout` | the two halves of the ABI cross-check against the C headers |

The generator panics on a registry shape it does not know instead of emitting something plausible. See [docs/design.md](../../docs/design.md) (Japanese).
