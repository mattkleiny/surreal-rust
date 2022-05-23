use crate::audio::{AudioHandle, AudioServerImpl};

/// The audio server for the desktop platform.
pub struct DesktopAudioServerImpl {}

impl DesktopAudioServerImpl {
  pub fn new() -> Self {
    Self {}
  }
}

impl AudioServerImpl for DesktopAudioServerImpl {
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
