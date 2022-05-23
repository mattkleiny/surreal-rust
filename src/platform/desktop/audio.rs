use crate::audio::{AudioHandle, AudioServer};

/// The audio server for the desktop platform.
pub struct DesktopAudioServer {}

impl DesktopAudioServer {
  pub fn new() -> Self {
    Self {}
  }
}

impl AudioServer for DesktopAudioServer {
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
