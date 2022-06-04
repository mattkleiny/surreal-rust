//! The OpenAL backend implementation for the audio subsystem.

use super::*;

/// An OpenAL [`AudioBackend`] implementation.
pub struct OpenALAudioBackend {}

impl OpenALAudioBackend {
  /// Creates a new OpenAL backend.
  pub fn new() -> Self {
    Self {}
  }
}

impl AudioBackend for OpenALAudioBackend {
  fn create_clip(&self) -> AudioHandle {
    todo!()
  }

  fn upload_clip_data(&self, _handle: AudioHandle, _data: &[u8]) {
    todo!()
  }

  fn delete_clip(&self, _handle: AudioHandle) {
    todo!()
  }
}
