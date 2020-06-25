pub use sdl2::keyboard::Keycode as Key; // TODO: create my own representation.

pub trait InputServer {
  fn is_key_pressed(&self, key: Key) -> bool;
}

