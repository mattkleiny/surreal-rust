//! Input/output utilities and virtual file system.

pub use packed::*;
#[cfg(feature = "serde")]
pub use serialization::*;
pub use streams::*;
pub use virtualfs::*;

mod packed;
#[cfg(feature = "serde")]
mod serialization;
mod streams;
mod virtualfs;
