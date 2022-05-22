use crate::input::{InputServer, KeyboardDevice, MouseDevice};

/// The input server for the headless platform.
pub struct HeadlessInputServer {}

impl HeadlessInputServer {
  pub fn new() -> Self {
    Self {}
  }
}

unsafe impl InputServer for HeadlessInputServer {
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