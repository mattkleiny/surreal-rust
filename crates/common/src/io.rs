//! Input/output utilities and virtual file system.

pub use buffers::*;
#[cfg(feature = "serde")]
pub use serde::*;
pub use streams::*;
pub use virtualfs::*;

mod buffers;
#[cfg(feature = "serde")]
mod serde;
mod streams;
mod virtualfs;
