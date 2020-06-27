use crate::input::*;

use super::DesktopPlatform;

impl InputServer for DesktopPlatform {
  fn is_key_up(&self, key: Key) -> bool {
    unimplemented!()
  }

  fn is_key_down(&self, key: Key) -> bool {
    unimplemented!()
  }

  fn is_key_pressed(&self, key: Key) -> bool {
    self.pressed_keys.contains(&key)
  }
}
