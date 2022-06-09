//! A headless audio backend for testing and etc.

use std::sync::atomic::{AtomicU32, Ordering};

use super::*;

/// A headless [`AudioBackend`] implementation.
///
/// This backend does nothing (no-ops) and can be used for testing/etc.
pub struct HeadlessAudioBackend {
  next_clip_id: AtomicU32,
  next_source_id: AtomicU32,
}

impl HeadlessAudioBackend {
  pub fn new() -> Self {
    Self {
      next_clip_id: AtomicU32::new(0),
      next_source_id: AtomicU32::new(0),
    }
  }
}

impl AudioBackend for HeadlessAudioBackend {
  fn create_clip(&self) -> AudioHandle {
    self.next_clip_id.fetch_add(1, Ordering::Relaxed)
  }

  fn upload_clip_data(&self, _handle: AudioHandle, _data: *const u8, _length: usize) {
    // no-op
  }

  fn delete_clip(&self, _handle: AudioHandle) {
    // no-op
  }

  fn create_source(&self) -> AudioHandle {
    self.next_source_id.fetch_add(1, Ordering::Relaxed)
  }

  fn is_source_playing(&self, _source: AudioHandle) -> bool {
    false
  }

  fn get_source_volume(&self, _source: AudioHandle) -> f32 {
    1.
  }

  fn set_source_volume(&self, _source: AudioHandle, _volume: f32) {
    // no-op
  }

  fn delete_source(&self, _handle: AudioHandle) {
    // no-op
  }
}
