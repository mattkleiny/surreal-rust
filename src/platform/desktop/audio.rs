use super::*;

/// The audio server for the desktop platform.
pub struct DesktopAudioServer {}

impl DesktopAudioServer {
  pub fn new() -> Self {
    Self {}
  }
}

unsafe impl AudioServer for DesktopAudioServer {
  unsafe fn create_clip(&self) -> AudioHandle {
    todo!()
  }

  unsafe fn upload_clip_data(&self, handle: AudioHandle, data: &[u8]) {
    todo!()
  }

  unsafe fn delete_clip(&self, handle: AudioHandle) {
    todo!()
  }
}
