use crate::audio::*;
use crate::RID;

use super::DesktopPlatform;

impl AudioServer for DesktopPlatform {
  fn create_audio_source(&mut self) -> Result<RID, AudioError> {
    unimplemented!()
  }
}
