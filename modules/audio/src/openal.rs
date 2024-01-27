//! The OpenAL backend implementation for the audio subsystem.

use super::*;

/// A OpenAL-based [`AudioBackend`] implementation.
pub struct OpenALAudioBackend {}

impl OpenALAudioBackend {
  /// Creates a new OpenAL graphics backend.
  pub fn new() -> Self {
    todo!()
  }
}

#[allow(unused_variables)]
impl AudioBackend for OpenALAudioBackend {
  fn new_audio_device(&self) -> Box<dyn AudioDevice> {
    todo!()
  }

  fn new_audio_recorder(&self) -> Box<dyn AudioRecorder> {
    todo!()
  }
}
