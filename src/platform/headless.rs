//! A platform implementation for headless environments.

use audio::HeadlessAudioBackend;
use graphics::HeadlessGraphicsBackend;
use input::HeadlessInput;

use crate::audio::AudioServer;
use crate::graphics::GraphicsServer;

use super::*;

mod audio;
mod graphics;
mod input;

/// A platform for headless environments.
pub struct HeadlessPlatform;

/// A host for headless environments.
pub struct HeadlessPlatformHost {
  pub audio: AudioServer<HeadlessAudioBackend>,
  pub graphics: GraphicsServer,
  pub input: HeadlessInput,
  is_exiting: bool,
}

impl Platform for HeadlessPlatform {
  type Host = HeadlessPlatformHost;

  fn create_host(&self) -> Self::Host {
    HeadlessPlatformHost {
      audio: HeadlessAudioBackend::new(),
      graphics: HeadlessGraphicsBackend::new(),
      input: HeadlessInput::new(),
      is_exiting: false,
    }
  }
}

impl PlatformHost for HeadlessPlatformHost {
  fn width(&self) -> usize {
    1920
  }

  fn height(&self) -> usize {
    1080
  }

  fn is_focused(&self) -> bool {
    true
  }

  fn is_closing(&self) -> bool {
    false
  }

  fn run(&mut self, mut body: impl FnMut(&mut Self)) {
    while !self.is_exiting {
      body(self);
    }
  }

  fn exit(&mut self) {
    self.is_exiting = true;
  }
}