//! Input/output utilities and virtual file system.

#[cfg(feature = "serde")]
pub use serde::*;
pub use streams::*;
pub use virtualfs::*;

#[cfg(feature = "serde")]
mod serde;
mod streams;
mod virtualfs;
