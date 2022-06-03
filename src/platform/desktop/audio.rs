use crate::audio::*;

/// The audio server for the desktop platform.
pub struct DesktopAudioBackend {}

impl DesktopAudioBackend {
  pub fn new() -> Self {
    Self {}
  }
}

impl AudioBackend for DesktopAudioBackend {
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
