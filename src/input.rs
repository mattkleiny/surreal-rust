//! A lightweight and cross-platform input engine.

pub use winit::event::MouseButton as MouseButton;
pub use winit::event::VirtualKeyCode as Key;

/// Represents a fallible result in the input subsystem.
pub type InputResult<T> = anyhow::Result<T>;

/// A server for the underlying input subsystem.
///
/// Permits interaction with the underlying input API through unsafe lower-level abstraction.
pub unsafe trait InputServer {
  fn keyboard_devices(&self) -> &[&dyn KeyboardDevice];
  fn mouse_devices(&self) -> &[&dyn MouseDevice];

  fn primary_keyboard_device(&self) -> Option<&dyn KeyboardDevice>;
  fn primary_mouse_device(&self) -> Option<&dyn MouseDevice>;
}

/// Abstracts over a keyboard device in the system.
pub trait KeyboardDevice {
  fn is_key_up(&self, key: Key) -> bool;
  fn is_key_down(&self, key: Key) -> bool;
  fn is_key_pressed(&self, key: Key) -> bool;
}

/// Abstracts over a mouse device in the system.
pub trait MouseDevice {
  fn is_button_up(&self, button: MouseButton) -> bool;
  fn is_button_down(&self, button: MouseButton) -> bool;
  fn is_button_pressed(&self, button: MouseButton) -> bool;
}
