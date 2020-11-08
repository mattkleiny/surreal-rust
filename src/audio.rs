//! A lightweight and fast cross-platform audio engine.

pub type AudioResult<T> = std::result::Result<T, Error>;

/// Abstracts over an audio device.
///
/// Permits interaction with the underlying audio API through a higher-level abstraction.
pub trait Audio {}

/// Represents an error with audio.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
  NotEnoughMemory,
}

impl From<Error> for crate::Error {
  fn from(error: Error) -> Self {
    Self::Audio(error)
  }
}
