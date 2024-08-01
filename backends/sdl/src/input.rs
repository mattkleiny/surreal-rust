//! Input handling for SDL.

pub use input::*;
use sdl2_sys::{SDL_KeyCode, SDL_Keycode};

/// A keyboard device for SDL.
#[derive(Default)]
pub struct SdlKeyboardDevice {
  events: Vec<KeyboardEvent>,
}

impl KeyboardDevice for SdlKeyboardDevice {
  fn events(&self) -> &[KeyboardEvent] {
    &self.events
  }
}

impl SdlKeyboardDevice {
  pub fn on_key_down(&mut self, scancode: SDL_Keycode) {
    if let Some(virtual_key) = convert_scancode(scancode) {
      self.events.push(KeyboardEvent::KeyDown(virtual_key))
    }
  }

  pub fn on_key_up(&mut self, scancode: SDL_Keycode) {
    if let Some(virtual_key) = convert_scancode(scancode) {
      self.events.push(KeyboardEvent::KeyUp(virtual_key))
    }
  }

  pub fn clear_events(&mut self) {
    self.events.clear();
  }
}

/// A mouse device for SDL.
#[derive(Default)]
pub struct SdlMouseDevice {
  events: Vec<MouseEvent>,
}

impl MouseDevice for SdlMouseDevice {
  fn events(&self) -> &[MouseEvent] {
    &self.events
  }
}

impl SdlMouseDevice {
  pub fn on_mouse_down(&mut self, button: u8) {
    if let Some(mouse_button) = match button {
      1 => Some(MouseButton::Left),
      2 => Some(MouseButton::Middle),
      3 => Some(MouseButton::Right),
      _ => None,
    } {
      self.events.push(MouseEvent::MouseDown(mouse_button));
    }
  }

  pub fn on_mouse_up(&mut self, button: u8) {
    if let Some(mouse_button) = match button {
      1 => Some(MouseButton::Left),
      2 => Some(MouseButton::Middle),
      3 => Some(MouseButton::Right),
      _ => None,
    } {
      self.events.push(MouseEvent::MouseUp(mouse_button));
    }
  }

  pub fn clear_events(&mut self) {
    self.events.clear();
  }
}

/// Converts an SDL scancode to a virtual key.
fn convert_scancode(scan_code: SDL_Keycode) -> Option<VirtualKey> {
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
