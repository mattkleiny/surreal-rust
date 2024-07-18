//! Input handling for SDL.

pub use input::*;

/// A keyboard device for SDL.
pub struct SdlKeyboardDevice {}

impl KeyboardDevice for SdlKeyboardDevice {
  fn is_key_down(&self, _key: VirtualKey) -> bool {
    todo!()
  }

  fn is_key_up(&self, _key: VirtualKey) -> bool {
    todo!()
  }
}

/// A mouse device for SDL.
pub struct SdlMouseDevice {}

impl MouseDevice for SdlMouseDevice {}
