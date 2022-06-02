//! A headless input backend for testing and etc.

use crate::input::*;

/// A headless [`InputBackend`] implementation.
///
/// This backend does nothing (no-ops) and can be used for testing/etc.
pub struct HeadlessInputBackend {}

impl HeadlessInputBackend {
  pub fn new() -> Self {
    Self {}
  }
}

impl InputBackend for HeadlessInputBackend {
  fn keyboard_device(&self) -> Option<&dyn KeyboardDevice> {
    None
  }

  fn mouse_device(&self) -> Option<&dyn MouseDevice> {
    None
  }
}