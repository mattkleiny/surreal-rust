use std::sync::atomic::{AtomicU32, Ordering};

use super::*;

/// A headless [`AudioBackend`] implementation.
///
/// This backend does nothing (no-ops) and can be used for testing/etc.
pub struct HeadlessAudioBackend {
  next_clip_id: AtomicU32,
}

impl HeadlessAudioBackend {
  pub fn new() -> AudioServer {
    AudioServer::new(Box::new(Self {
      next_clip_id: AtomicU32::new(0),
    }))
  }
}

impl AudioBackend for HeadlessAudioBackend {
  fn create_clip(&self) -> AudioHandle {
    self.next_clip_id.fetch_add(1, Ordering::Relaxed)
  }

  fn upload_clip_data(&self, _handle: AudioHandle, _data: &[u8]) {
    // no-op
  }

  fn delete_clip(&self, _handle: AudioHandle) {
    // no-op
  }
}
