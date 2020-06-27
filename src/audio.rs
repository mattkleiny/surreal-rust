//! A lightweight and fast cross-platform audio engine.

use crate::RID;

pub trait AudioServer {
  fn create_audio_source(&mut self) -> Result<RID, AudioError>;
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AudioError {
  NotEnoughMemory,
}
