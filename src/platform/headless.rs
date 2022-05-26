//! A platform implementation for headless environments.

use crate::audio::{AudioServer, HeadlessAudioBackend};
use crate::graphics::{GraphicsServer, HeadlessGraphicsBackend};
use crate::input::HeadlessInput;

use super::*;

/// A platform for headless environments.
pub struct HeadlessPlatform;

/// A host for headless environments.
pub struct HeadlessPlatformHost {
  pub audio: AudioServer,
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

  fn audio(&self) -> &AudioServer {
    &self.audio
  }

  fn graphics(&self) -> &GraphicsServer {
    &self.graphics
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