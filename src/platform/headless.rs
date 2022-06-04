//! A platform implementation for headless environments.

use crate::audio::{AudioServer, HeadlessAudioBackend};
use crate::framework::EventListener;
use crate::graphics::{GraphicsServer, HeadlessGraphicsBackend};
use crate::input::HeadlessInputBackend;

use super::*;

/// A platform for headless environments.
pub struct HeadlessPlatform;

/// A host for headless environments.
pub struct HeadlessPlatformHost {
  audio: AudioServer,
  graphics: GraphicsServer,
  input: HeadlessInputBackend,
  is_exiting: bool,
}

impl Platform for HeadlessPlatform {
  type Host = HeadlessPlatformHost;

  fn create_host(&self) -> Self::Host {
    HeadlessPlatformHost {
      audio: AudioServer::new(Box::new(HeadlessAudioBackend::new())),
      graphics: GraphicsServer::new(Box::new(HeadlessGraphicsBackend::new())),
      input: HeadlessInputBackend::new(),
      is_exiting: false,
    }
  }
}

impl PlatformHost for HeadlessPlatformHost {
  fn audio(&self) -> &AudioServer {
    &self.audio
  }

  fn graphics(&self) -> &GraphicsServer {
    &self.graphics
  }

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

  fn pump(&mut self, mut listener: impl EventListener + 'static) {
    use crate::framework::*;

    while !self.is_exiting {
      listener.on_event(&PlatformTickEvent());
      listener.on_event(&PlatformRenderEvent());
    }
  }

  fn exit(&mut self) {
    self.is_exiting = true;
  }
}
