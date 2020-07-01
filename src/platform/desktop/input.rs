use crate::input::*;

use super::DesktopPlatform;

impl Input for DesktopPlatform {
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
  fn from(_: winit::event::MouseButton) -> Self {
    // TODO: implement me
    MouseButton::Left
  }
}

impl From<winit::event::ScanCode> for Key {
  fn from(_: u32) -> Self {
    // TODO: implement me
    Key::Escape
  }
}
