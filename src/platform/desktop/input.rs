use crate::input::*;

use super::DesktopPlatform;

impl InputDevice for DesktopPlatform {
  fn is_button_up(&self, button: MouseButton) -> bool {
    !self.pressed_buttons.contains(&button)
  }

  fn is_button_down(&self, button: MouseButton) -> bool {
    self.pressed_buttons.contains(&button)
  }

  fn is_button_pressed(&self, button: MouseButton) -> bool {
    self.pressed_buttons.contains(&button)
  }

  fn is_key_up(&self, key: Key) -> bool {
    !self.pressed_keys.contains(&key)
  }

  fn is_key_down(&self, key: Key) -> bool {
    self.pressed_keys.contains(&key)
  }

  fn is_key_pressed(&self, key: Key) -> bool {
    self.pressed_keys.contains(&key)
  }

  fn get_active_touches(&self) -> &[Touch] {
    unimplemented!()
  }
}

impl From<winit::event::MouseButton> for MouseButton {
  fn from(button: winit::event::MouseButton) -> Self {
    match button {
      winit::event::MouseButton::Left => Self::Left,
      winit::event::MouseButton::Right => Self::Right,
      winit::event::MouseButton::Middle => Self::Middle,
      winit::event::MouseButton::Other(_) => Self::Middle,
    }
  }
}

impl From<winit::event::ScanCode> for Key {
  fn from(code: u32) -> Self {
    Self::from_scan_code(code)
  }
}
