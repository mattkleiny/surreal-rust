//! A lightweight and fast cross-platform audio engine.

use crate::RID;

/// Abstracts over an audio device.
///
/// Permits interaction with the underlying audio API through a higher-level abstraction.
pub trait AudioDevice {
  fn create_audio_source(&mut self) -> Result<RID, AudioError>;
}

/// Represents an error with audio.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AudioError {
  NotEnoughMemory,
}
