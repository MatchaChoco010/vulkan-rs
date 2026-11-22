#![no_std]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod device;
mod entry;
mod instance;
mod loader;
pub mod vk;

pub use device::Device;
pub use entry::{Entry, StaticFn};
pub use instance::Instance;
#[cfg(feature = "std")]
pub use loader::LoadError;
pub(crate) use loader::load_pfn;
