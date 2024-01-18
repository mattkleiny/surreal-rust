//! Input/output utilities and virtual file system.

pub use serialization::*;
pub use streams::*;
pub use virtualfs::*;

mod serialization;
mod streams;
mod virtualfs;

// TODO: implement packed file system support
