//! The OpenAL backend implementation for the audio subsystem.

use super::*;

/// An [`AudioBackend`] implementation for OpenAL.
pub struct OpenALAudioBackend {}

impl OpenALAudioBackend {
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

