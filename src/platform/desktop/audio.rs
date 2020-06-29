use crate::audio::*;
use crate::RID;

use super::DesktopPlatform;

impl AudioDevice for DesktopPlatform {
  fn create_audio_source(&mut self) -> Result<RID, AudioError> {
    unimplemented!()
  }
}
