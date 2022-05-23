use crate::audio::*;

/// The audio server for the desktop platform.
pub struct DesktopAudio {}

impl DesktopAudio {
  pub fn new() -> AudioServer<Self> {
    AudioServer::new(Self {})
  }
}

impl AudioImpl for DesktopAudio {
  type Handle = u32;

  fn create_clip(&self) -> Self::Handle {
    todo!()
  }

  fn upload_clip_data(&self, _handle: Self::Handle, _data: &[u8]) {
    todo!()
  }

  fn delete_clip(&self, _handle: Self::Handle) {
    todo!()
  }
}
