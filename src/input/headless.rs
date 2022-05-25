use crate::input::*;

/// A headless [`InputBackend`] implementation.
///
/// This backend does nothing (no-ops) and can be used for testing/etc.
pub struct HeadlessInput {}

impl HeadlessInput {
  pub fn new() -> Self {
    Self {}
  }
}

impl InputBackend for HeadlessInput {
  fn keyboard_device(&self) -> Option<&dyn KeyboardDevice> {
    None
  }

  fn mouse_device(&self) -> Option<&dyn MouseDevice> {
    None
  }
}