use crate::input::*;

/// The input server for the headless platform.
pub struct HeadlessInput {}

impl HeadlessInput {
  pub fn new() -> Self {
    Self {}
  }
}

impl InputServerImpl for HeadlessInput {
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