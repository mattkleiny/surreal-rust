//! Input support for the GameBoy Advance.

use crate::GameBoyRuntime;

/// A button on the GameBoy Advance.
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Button {
  A,
  B,
  Start,
  Select,
  Up,
  Down,
  Left,
  Right,
}

/// Represents the input of the GameBoy Advance.
pub trait InputDevice {
  fn is_button_down(&self, button: Button) -> bool;
  fn is_button_up(&self, button: Button) -> bool;
  fn is_button_pressed(&self, button: Button) -> bool;
  fn is_button_released(&self, button: Button) -> bool;
  fn is_button_held(&self, button: Button) -> bool;
  fn is_button_not_held(&self, button: Button) -> bool;
}

impl InputDevice for GameBoyRuntime {
  fn is_button_down(&self, button: Button) -> bool {
    todo!()
  }

  fn is_button_up(&self, button: Button) -> bool {
    todo!()
  }

  fn is_button_pressed(&self, button: Button) -> bool {
    todo!()
  }

  fn is_button_released(&self, button: Button) -> bool {
    todo!()
  }

  fn is_button_held(&self, button: Button) -> bool {
    todo!()
  }

  fn is_button_not_held(&self, button: Button) -> bool {
    todo!()
  }
}
