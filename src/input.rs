use crate::maths::Vector2;

pub trait InputServer {
  fn is_key_up(&self, key: Key) -> bool;
  fn is_key_down(&self, key: Key) -> bool;
  fn is_key_pressed(&self, key: Key) -> bool;
  fn get_active_touches(&self) -> &[Touch];
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Key {
  Space,
  Escape,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Touch {
  pos: Vector2<u32>
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum InputError {}

impl From<InputError> for crate::Error {
  fn from(_: InputError) -> Self {
    crate::Error::Input
  }
}
