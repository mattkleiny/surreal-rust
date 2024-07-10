use std::sync::atomic::{AtomicU64, Ordering};

use super::*;

/// A headless [`AudioBackend`] implementation.
///
/// This backend does nothing (no-ops) and can be used for testing/etc.
#[derive(Default)]
pub struct HeadlessAudioBackend {
  next_clip_id: AtomicU64,
  next_source_id: AtomicU64,
}

#[allow(unused_variables)]
impl AudioBackend for HeadlessAudioBackend {
  fn clip_create(&self) -> Result<ClipId, ClipError> {
    Ok(ClipId(self.next_clip_id.fetch_add(1, Ordering::Relaxed)))
  }

  fn clip_write_data(&self, clip: ClipId, data: *const u8, length: usize) -> Result<(), ClipError> {
    Ok(())
  }

  fn clip_delete(&self, clip: ClipId) -> Result<(), ClipError> {
    Ok(())
  }

  fn source_create(&self) -> Result<SourceId, SourceError> {
    Ok(SourceId(self.next_source_id.fetch_add(1, Ordering::Relaxed)))
  }

  fn source_is_playing(&self, source: SourceId) -> Option<bool> {
    None
  }

  fn source_get_volume(&self, source: SourceId) -> Option<f32> {
    None
  }

  fn source_set_volume(&self, source: SourceId, volume: f32) -> Result<(), SourceError> {
    Ok(())
  }

  fn source_get_clip(&self, source: SourceId) -> Option<ClipId> {
    None
  }

  fn source_set_clip(&self, source: SourceId, clip: ClipId) -> Result<(), SourceError> {
    Ok(())
  }

  fn source_play(&self, source: SourceId) -> Result<(), SourceError> {
    Ok(())
  }

  fn source_delete(&self, source: SourceId) -> Result<(), SourceError> {
    Ok(())
  }
}
