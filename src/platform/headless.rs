//! A headless platform for Surreal.

use crate::audio::SoundClip;
use crate::graphics::Color;
use crate::input::Keycode;

use super::*;

/// A platform without visible output.
pub struct HeadlessPlatform;

impl Platform for HeadlessPlatform {
  type Host = HeadlessHost;
  type Allocator = PortableAllocator;
  type FileSystem = PortableFileSystem;
  type AudioDevice = HeadlessHost;
  type GraphicsDevice = HeadlessHost;
  type InputDevice = HeadlessHost;

  fn build(&self) -> Result<Self::Host, super::PlatformError> {
    Ok(HeadlessHost)
  }
}

/// A host for the headless platform.
pub struct HeadlessHost;

impl Host for HeadlessHost {
  fn width(&self) -> u32 {
    1920
  }

  fn height(&self) -> u32 {
    1080
  }

  fn is_closing(&self) -> bool {
    false
  }

  fn tick<C>(&mut self, mut callback: C)
  where
    C: FnMut(&mut Self, DeltaTime) -> (),
  {
    callback(self, 1.); // just invoke the callback as quickly as possible
  }

  fn exit(&mut self) {}
}

impl AudioDevice for HeadlessHost {
  fn play(&mut self, _clip: &SoundClip) {}
}

impl GraphicsDevice for HeadlessHost {
  fn clear(&mut self, _color: Color) {}
}

impl InputDevice for HeadlessHost {
  fn is_pressed(&self, _binding: impl Into<Keycode>) -> bool {
    false
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_should_create_a_headless_platform() {
    let mut host = HeadlessPlatform.build().unwrap();

    host.tick(|_host, _delta_time| {});
  }
}
