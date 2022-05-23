use std::sync::atomic::{AtomicU32, Ordering};

use crate::audio::{AudioHandle, AudioServerImpl};

/// The audio server for the headless platform.
pub struct HeadlessAudioServer {
  next_clip_id: AtomicU32,
}

impl HeadlessAudioServer {
  pub fn new() -> Self {
    Self {
      next_clip_id: AtomicU32::new(0),
    }
  }
}

impl AudioServerImpl for HeadlessAudioServer {
  fn create_clip(&self) -> AudioHandle {
    AudioHandle { id: self.next_clip_id.fetch_add(1, Ordering::Relaxed) }
  }

  fn upload_clip_data(&self, _handle: AudioHandle, _data: &[u8]) {
    // no-op
  }

  fn delete_clip(&self, _handle: AudioHandle) {
    // no-op
  }
}
