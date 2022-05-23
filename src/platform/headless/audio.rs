use std::sync::atomic::{AtomicU32, Ordering};

use crate::audio::{AudioHandle, AudioImpl, AudioServer};

/// The audio server for the headless platform.
pub struct HeadlessAudio {
  next_clip_id: AtomicU32,
}

impl HeadlessAudio {
  pub fn new() -> AudioServer<Self> {
    AudioServer::new(Self {
      next_clip_id: AtomicU32::new(0),
    })
  }
}

impl AudioImpl for HeadlessAudio {
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
