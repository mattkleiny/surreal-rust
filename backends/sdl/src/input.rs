//! Input handling for SDL.

use common::FastHashSet;
pub use input::*;

/// A keyboard device for SDL.
#[derive(Default)]
pub struct SdlKeyboardDevice {
  // TODO: clean this up
  pub(crate) keyboard_state: FastHashSet<VirtualKey>,
}

impl KeyboardDevice for SdlKeyboardDevice {
  fn is_key_down(&self, key: VirtualKey) -> bool {
    self.keyboard_state.contains(&key)
  }

  fn is_key_up(&self, key: VirtualKey) -> bool {
    !self.keyboard_state.contains(&key)
  }
}

/// A mouse device for SDL.
#[derive(Default)]
pub struct SdlMouseDevice {}

impl MouseDevice for SdlMouseDevice {}
