//! What the examples have in common and what none of them is about: creating the
//! instance with validation, picking a device, loading shaders, allocating and
//! uploading memory, one-time command submission, and writing the output image.
//!
//! Every function that calls Vulkan is `unsafe` and leaves the valid usage rules of the
//! commands it calls to its caller, as the commands themselves do.

pub mod commands;
pub mod device;
pub mod extensions;
#[cfg(test)]
mod fake;
pub mod image;
pub mod instance;
pub mod memory;
pub mod output;
#[cfg(test)]
mod ownership_tests;
pub mod pipeline;
pub mod shader;
#[cfg(feature = "window")]
pub mod window;
