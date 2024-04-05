//! The OpenAL backend implementation for the audio subsystem.

use common::profiling;

use super::*;

/// A OpenAL-based [`AudioBackend`] implementation.
pub struct OpenALAudioBackend {}

impl OpenALAudioBackend {
  /// Creates a new OpenAL graphics backend.
  pub fn new() -> Self {
    todo!()
  }
}

impl Drop for OpenALAudioBackend {
  fn drop(&mut self) {
    todo!()
  }
}

#[allow(unused_variables)]
impl AudioBackend for OpenALAudioBackend {
  #[profiling]
  fn clip_create(&self) -> Result<ClipId, ClipError> {
    todo!()
  }

  #[profiling]
  fn clip_write_data(&self, clip: ClipId, data: *const u8, length: usize) -> Result<(), ClipError> {
    todo!()
  }

  #[profiling]
  fn clip_delete(&self, clip: ClipId) -> Result<(), ClipError> {
    todo!()
  }

  #[profiling]
  fn source_create(&self) -> Result<SourceId, SourceError> {
    todo!()
  }

  #[profiling]
  fn source_is_playing(&self, source: SourceId) -> Option<bool> {
    todo!()
  }

  #[profiling]
  fn source_get_volume(&self, source: SourceId) -> Option<f32> {
    todo!()
  }

  #[profiling]
  fn source_set_volume(&self, source: SourceId, volume: f32) -> Result<(), SourceError> {
    todo!()
  }

  #[profiling]
  fn source_get_clip(&self, source: SourceId) -> Option<ClipId> {
    todo!()
  }

  #[profiling]
  fn source_set_clip(&self, source: SourceId, clip: ClipId) -> Result<(), SourceError> {
    todo!()
  }

  #[profiling]
  fn source_play(&self, source: SourceId) -> Result<(), SourceError> {
    todo!()
  }

  #[profiling]
  fn source_delete(&self, source: SourceId) -> Result<(), SourceError> {
    todo!()
  }
}
