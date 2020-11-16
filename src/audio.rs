//! A lightweight and fast cross-platform audio engine.

pub use clips::*;

mod clips;

pub type AudioResult<T> = std::result::Result<T, Error>;

/// Abstracts over an audio device.
///
/// Permits interaction with the underlying audio API through a higher-level abstraction.
pub trait AudioDevice {}

/// Represents an error with audio.
#[derive(Debug)]
pub enum Error {
  NotEnoughMemory,
}

impl From<Error> for crate::Error {
  fn from(error: Error) -> Self {
    Self::Audio(error)
  }
}
