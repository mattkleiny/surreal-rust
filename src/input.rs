//! A lightweight and cross-platform input engine.

pub use winit::event::MouseButton as MouseButton;
pub use winit::event::VirtualKeyCode as Key;

pub use headless::*;

use crate::maths::Vector2;

mod headless;

/// A backend for the underlying input subsystem.
///
/// Permits interaction with the underlying input API through lower-level abstraction.
pub trait InputBackend {
  fn keyboard_device(&self) -> Option<&dyn KeyboardDevice>;
  fn mouse_device(&self) -> Option<&dyn MouseDevice>;
}

/// Abstracts over a keyboard device in the system.
pub trait KeyboardDevice {
  fn is_key_up(&self, key: Key) -> bool;
  fn is_key_down(&self, key: Key) -> bool;
  fn is_key_pressed(&self, key: Key) -> bool;
}

/// Abstracts over a mouse device in the system.
pub trait MouseDevice {
  /// Gets the position of the mouse relative to the window.
  fn position(&self) -> Vector2<f32>;

  /// Gets the position of the mouse normalised to the window size.
  fn normalised_position(&self) -> Vector2<f32>;

  fn is_button_up(&self, button: MouseButton) -> bool;
  fn is_button_down(&self, button: MouseButton) -> bool;
  fn is_button_pressed(&self, button: MouseButton) -> bool;
}
