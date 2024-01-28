//! Input/output utilities and virtual file system.

pub use binary::*;
#[cfg(feature = "serde")]
pub use serde::*;
pub use streams::*;
pub use virtualfs::*;

mod binary;
#[cfg(feature = "serde")]
mod serde;
mod streams;
mod virtualfs;
