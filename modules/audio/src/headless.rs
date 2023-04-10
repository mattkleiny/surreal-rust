use super::*;

/// A headless [`AudioBackend`] implementation.
///
/// This backend does nothing (no-ops) and can be used for testing/etc.
#[derive(Default)]
pub struct HeadlessAudioBackend {}

#[allow(unused_variables)]
impl AudioBackend for HeadlessAudioBackend {
  fn clip_create(&self) -> AudioClipId {
    todo!()
  }

  fn clip_write_data(&self, clip: AudioClipId, data: *const u8, length: usize) {
    todo!()
  }

  fn clip_delete(&self, clip: AudioClipId) {
    todo!()
  }

  fn source_create(&self) -> AudioSourceId {
    todo!()
  }

  fn source_is_playing(&self, source: AudioSourceId) -> bool {
    todo!()
  }

  fn source_get_volume(&self, source: AudioSourceId) -> f32 {
    todo!()
  }

  fn source_set_volume(&self, source: AudioSourceId, volume: f32) {
    todo!()
  }

  fn source_delete(&self, source: AudioSourceId) {
    todo!()
  }

  fn source_get_clip(&self, source: AudioSourceId) -> Option<AudioClipId> {
    todo!()
  }

  fn source_set_clip(&self, source: AudioSourceId, clip: AudioClipId) {
    todo!()
  }

  fn source_play(&self, source: AudioSourceId) {
    todo!()
  }
}
