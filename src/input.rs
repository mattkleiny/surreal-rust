//! A lightweight and fast cross-platform input engine.

/// Represents a fallible result in the input subsystem.
pub type InputResult<T> = anyhow::Result<T>;

/// A server for the underlying input subsystem.
///
/// Permits interaction with the underlying input API through unsafe lower-level abstraction.
pub unsafe trait InputServer {
  /// Retrieves a list of all attached `KeyboardDevice`s.
  fn keyboard_devices(&self) -> &[&dyn KeyboardDevice];

  /// Retrieves a list of all attached `MouseDevice`s.
  fn mouse_devices(&self) -> &[&dyn MouseDevice];
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

/// Represents a button on the mouse.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MouseButton {
  Left,
  Middle,
  Right,
}

/// Represents a key on the keyboard.
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Key {
  Space,
  Escape,
}

impl Key {
  pub fn from_scan_code(scancode: u32) -> Self {
    // TODO: replace this with something safe
    unsafe { std::mem::transmute(scancode) }
  }
}

