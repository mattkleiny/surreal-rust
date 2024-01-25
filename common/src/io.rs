//! Input/output utilities and virtual file system.

pub use packed::*;
#[cfg(feature = "serde")]
pub use serde::*;
pub use streams::*;
pub use virtualfs::*;

mod packed;
#[cfg(feature = "serde")]
mod serde;
mod streams;
mod virtualfs;
