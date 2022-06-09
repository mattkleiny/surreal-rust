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

impl Drop for OpenALAudioBackend {
  fn drop(&mut self) {}
}

impl AudioBackend for OpenALAudioBackend {
  fn create_clip(&self) -> AudioHandle {
    todo!()
  }

  fn upload_clip_data(&self, _handle: AudioHandle, _data: *const u8, _length: usize) {
    todo!()
  }

  fn delete_clip(&self, _handle: AudioHandle) {
    todo!()
  }

  fn create_source(&self) -> AudioHandle {
    todo!()
  }

  fn is_source_playing(&self, _source: AudioHandle) -> bool {
    todo!()
  }

  fn get_source_volume(&self, _source: AudioHandle) -> f32 {
    todo!()
  }

  fn set_source_volume(&self, _source: AudioHandle, _volume: f32) {
    todo!()
  }

  fn delete_source(&self, _handle: AudioHandle) {
    todo!()
  }
}
