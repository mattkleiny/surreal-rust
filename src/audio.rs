//! A lightweight and fast cross-platform audio engine.

use crate::RID;

pub type AudioResult<T> = std::result::Result<T, AudioError>;

/// Abstracts over an audio device.
///
/// Permits interaction with the underlying audio API through a higher-level abstraction.
pub trait Audio {
  fn create_audio_source(&mut self) -> AudioResult<RID>;
}

/// Represents an error with audio.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AudioError {
  NotEnoughMemory,
}
