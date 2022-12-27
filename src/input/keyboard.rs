//! Keyboard input management.

use std::collections::HashSet;

pub use glutin::event::VirtualKeyCode as Key;
use glutin::event::{ElementState, KeyboardInput};

/// Represents a keyboard device in the system.
#[derive(Default)]
pub struct KeyboardDevice {
  previous_keys: HashSet<Key>,
  current_keys: HashSet<Key>,
}

impl KeyboardDevice {
  /// Ticks the keyboard device.
  pub fn tick(&mut self) {
    self.previous_keys.clear();
  }

  /// Handles keyboard events.
  pub fn on_keyboard_event(&mut self, event: &KeyboardInput) {
    if let Some(virtual_key_code) = event.virtual_keycode {
      if event.state == ElementState::Pressed {
        self.current_keys.insert(virtual_key_code);
        self.previous_keys.insert(virtual_key_code);
      } else {
        self.current_keys.remove(&virtual_key_code);
      }
    }
  }

  /// Is the given key up?
  pub fn is_key_up(&self, key: Key) -> bool {
    !self.current_keys.contains(&key)
  }

  /// Is the given key down?
  pub fn is_key_down(&self, key: Key) -> bool {
    self.current_keys.contains(&key)
  }

  /// Is the given key pressed this frame?
  pub fn is_key_pressed(&self, key: Key) -> bool {
    self.previous_keys.contains(&key)
  }
}
