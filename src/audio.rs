//! A lightweight and fast cross-platform audio engine.

pub type AudioResult<T> = std::result::Result<T, AudioError>;

/// Abstracts over an audio device.
///
/// Permits interaction with the underlying audio API through a higher-level abstraction.
pub trait Audio {}

/// Represents an error with audio.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AudioError {
  NotEnoughMemory,
}
