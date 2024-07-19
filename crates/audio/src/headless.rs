use std::sync::atomic::{AtomicU64, Ordering};

use super::*;

/// A headless [`AudioBackend`] implementation.
///
/// This backend does nothing (no-ops) and can be used for testing/etc.
#[derive(Default)]
pub struct HeadlessAudioBackend {
  next_buffer_id: AtomicU64,
  next_clip_id: AtomicU64,
  next_source_id: AtomicU64,
}

#[allow(unused_variables)]
impl AudioBackend for HeadlessAudioBackend {
  fn buffer_create(&self) -> Result<BufferId, BufferError> {
    Ok(BufferId(self.next_buffer_id.fetch_add(1, Ordering::Relaxed)))
  }

  fn buffer_write_data(&self, buffer: BufferId, sample_rate: AudioSampleRate, data: &[u8]) -> Result<(), BufferError> {
    Ok(())
  }

  fn buffer_delete(&self, buffer: BufferId) -> Result<(), BufferError> {
    Ok(())
  }

  fn clip_create(&self) -> Result<ClipId, ClipError> {
    Ok(ClipId(self.next_clip_id.fetch_add(1, Ordering::Relaxed)))
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

  fn source_get_gain(&self, source: SourceId) -> Option<f32> {
    Some(1.0f32)
  }

  fn source_set_gain(&self, source: SourceId, gain: f32) -> Result<(), SourceError> {
    Ok(())
  }

  fn source_get_pitch(&self, source: SourceId) -> Option<f32> {
    Some(1.0f32)
  }

  fn source_set_pitch(&self, source: SourceId, pitch: f32) -> Result<(), SourceError> {
    Ok(())
  }

  fn source_get_position(&self, source: SourceId) -> Option<Vec3> {
    Some(Vec3::ZERO)
  }

  fn source_set_position(&self, source: SourceId, position: Vec3) -> Result<(), SourceError> {
    Ok(())
  }

  fn source_set_velocity(&self, source: SourceId, velocity: Vec3) -> Result<(), SourceError> {
    Ok(())
  }

  fn source_get_velocity(&self, source: SourceId) -> Option<Vec3> {
    Some(Vec3::ZERO)
  }

  fn source_is_looping(&self, source: SourceId) -> Option<bool> {
    Some(false)
  }

  fn source_set_looping(&self, source: SourceId, looping: bool) -> Result<(), SourceError> {
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
