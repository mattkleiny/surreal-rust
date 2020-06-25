//! A lightweight and fast cross-platform audio engine.

use crate::core::RID;

pub trait AudioServer {
  fn create_audio_source(&mut self) -> Result<RID, AudioSourceError>;
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AudioSourceError {
  NotEnoughMemory,
}
