use crate::InputDevice;

/// A keyboard input device.
pub struct Keyboard {}

impl InputDevice for Keyboard {
  fn on_event(&mut self, _event: &crate::InputEvent) {
    todo!()
  }
}

/// Possible key codes on a keyboard.
pub enum KeyCode {}
