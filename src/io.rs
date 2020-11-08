//! Input/output abstractions and virtual file system.

pub use vfs::*;

mod vfs;

/// Represents an error in the IO subsystem.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
  VFS(vfs::Error),
}

impl From<Error> for crate::Error {
  fn from(error: Error) -> Self {
    Self::IO(error)
  }
}
