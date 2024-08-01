//! Input handling for SDL.

use common::FastHashSet;
pub use input::*;
use sdl2_sys::{SDL_KeyCode, SDL_Keycode};

/// A keyboard device for SDL.
#[derive(Default)]
pub struct SdlKeyboardDevice {
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

/// Converts an SDL scancode to a virtual key.
pub(crate) fn virtualkey_from_scancode(scan_code: SDL_Keycode) -> Option<VirtualKey> {
  use input::VirtualKey::*;

  match unsafe { std::mem::transmute::<SDL_Keycode, SDL_KeyCode>(scan_code) } {
    SDL_KeyCode::SDLK_ESCAPE => Some(Escape),
    SDL_KeyCode::SDLK_UP => Some(ArrowUp),
    SDL_KeyCode::SDLK_DOWN => Some(ArrowDown),
    SDL_KeyCode::SDLK_LEFT => Some(ArrowLeft),
    SDL_KeyCode::SDLK_RIGHT => Some(ArrowRight),
    _ => None,
  }
}
