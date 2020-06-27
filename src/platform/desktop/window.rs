use std::ffi::NulError;

use crate::window::*;

use super::DesktopPlatform;

impl WindowServer for DesktopPlatform {
  fn set_title(&mut self, title: impl AsRef<str>) -> Result<(), WindowError> {
    unimplemented!()
  }
}

impl From<NulError> for WindowError {
  fn from(_: NulError) -> Self {
    WindowError::GeneralFailure
  }
}
