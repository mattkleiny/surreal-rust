use crate::input::*;

use super::DesktopPlatform;

impl InputServer for DesktopPlatform {
  fn is_key_pressed(&self, key: Key) -> bool {
    self.pressed_keys.contains(&key)
  }
}
