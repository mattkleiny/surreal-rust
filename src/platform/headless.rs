//! A platform implementation for headless environments.

use winit::event_loop::ControlFlow;

use crate::audio::{AudioServer, HeadlessAudioBackend};
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
}

impl Platform for HeadlessPlatform {
  type Host = HeadlessPlatformHost;

  fn create_host(&self) -> Self::Host {
    HeadlessPlatformHost {
      audio: AudioServer::new(Box::new(HeadlessAudioBackend::new())),
      graphics: GraphicsServer::new(Box::new(HeadlessGraphicsBackend::new())),
      input: HeadlessInputBackend::new(),
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

  fn run(mut self, mut body: impl FnMut(&mut Self, &mut ControlFlow)) {
    let mut control_flow = ControlFlow::Poll;

    while control_flow != ControlFlow::Exit {
      body(&mut self, &mut control_flow);
    }
  }
}
