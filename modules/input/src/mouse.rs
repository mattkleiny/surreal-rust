use surreal::{collections::FastHashSet, maths::Vec2};

use crate::InputDevice;

/// A mouse input device.
pub struct Mouse {
  _cursor_pos: Vec2,
  _cursor_delta: Vec2,
  _pressed_buttons: FastHashSet<MouseButton>,
}

impl InputDevice for Mouse {
  fn on_event(&mut self, _event: &crate::InputEvent) {
    todo!()
  }
}

/// Possible mouse buttons.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
  Left,
  Right,
  Middle,
  Extra(u8),
}
