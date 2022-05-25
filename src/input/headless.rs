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
  fn keyboard_devices(&self) -> &[&dyn KeyboardDevice] {
    &[]
  }

  fn mouse_devices(&self) -> &[&dyn MouseDevice] {
    &[]
  }

  fn primary_keyboard_device(&self) -> Option<&dyn KeyboardDevice> {
    None
  }

  fn primary_mouse_device(&self) -> Option<&dyn MouseDevice> {
    None
  }
}