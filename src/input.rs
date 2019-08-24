//! A lightweight input system.

/// The supported key-codes for the input system.
pub use sdl2::keyboard::Keycode as Keycode;

/// An abstraction over the input device a system.
pub trait InputDevice {
  fn is_pressed(&self, binding: impl Into<Keycode>) -> bool;
}
