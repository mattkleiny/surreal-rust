use crate::audio::*;
use crate::RID;

use super::DesktopPlatform;

impl Audio for DesktopPlatform {
  fn create_audio_source(&mut self) -> Result<RID, AudioError> {
    unimplemented!()
  }
}
