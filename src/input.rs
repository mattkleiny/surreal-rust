//! A lightweight and fast cross-platform input engine.

use crate::maths::Vector2;

pub type InputResult<T> = std::result::Result<T, Error>;

/// Abstracts over the input system of a device.
///
/// Permits interaction with the underlying input API through a higher-level abstraction.
pub trait InputDevice {
  fn is_button_up(&self, button: MouseButton) -> bool;
  fn is_button_down(&self, button: MouseButton) -> bool;
  fn is_button_pressed(&self, button: MouseButton) -> bool;

  fn is_key_up(&self, key: Key) -> bool;
  fn is_key_down(&self, key: Key) -> bool;
  fn is_key_pressed(&self, key: Key) -> bool;

  fn get_active_touches(&self) -> &[Touch];
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

/// Represents a touch on a touch screen.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Touch {
  pos: Vector2<u32>,
}

/// Represents an error with input.
#[derive(Debug)]
pub enum Error {}

impl From<Error> for crate::Error {
  fn from(error: Error) -> Self {
    Self::Input(error)
  }
}
