use crate::audio::{AudioHandle, AudioImpl, AudioServer};

/// The audio server for the desktop platform.
pub struct DesktopAudio {}

impl DesktopAudio {
  pub fn new() -> AudioServer<Self> {
    AudioServer::new(Self {})
  }
}

impl AudioImpl for DesktopAudio {
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
