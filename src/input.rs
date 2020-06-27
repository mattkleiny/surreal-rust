pub use sdl2::keyboard::Keycode as Key;

// TODO: create my own representation.

pub trait InputServer {
  fn is_key_up(&self, key: Key) -> bool;
  fn is_key_down(&self, key: Key) -> bool;
  fn is_key_pressed(&self, key: Key) -> bool;
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum InputError {}

impl From<InputError> for crate::Error {
  fn from(_: InputError) -> Self {
    crate::Error::Input
  }
}