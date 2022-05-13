//! A platform implementation for desktop PCs.

use std::collections::HashSet;

use raw_gl_context::{GlConfig, GlContext};
use winit::dpi::LogicalSize;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

use audio::DesktopAudioServer;
use graphics::DesktopGraphicsServer;
use input::DesktopInputServer;

use crate::audio::AudioHandle;
use crate::graphics::{Color, GraphicsHandle, Viewport};
use crate::input::{Key, MouseButton};
use crate::maths::{vec2, Vector2};

use super::*;

mod audio;
mod graphics;
mod input;

/// Configuration for the `DesktopPlatform`.
#[derive(Copy, Clone, Debug)]
pub struct Configuration {
  pub title: &'static str,
  pub size: (u32, u32),
  pub update_continuously: bool,
}

impl Default for Configuration {
  fn default() -> Self {
    Self {
      title: "Surreal",
      size: (1920, 1080),
      update_continuously: true,
    }
  }
}

/// A `Platform` implementation for desktop PCs.
pub struct DesktopPlatform {
  // core
  config: Configuration,
  event_loop: EventLoop<()>,
  window: Window,

  // servers
  pub audio_server: DesktopAudioServer,
  pub graphics_server: DesktopGraphicsServer,
  pub input_server: DesktopInputServer,
}

impl DesktopPlatform {
  pub fn new(config: Configuration) -> Self {
    // prepare the main window and event loop
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(config.title)
        .with_inner_size(LogicalSize::new(config.size.0, config.size.1))
        .build(&event_loop)
        .unwrap();

    Self {
      // servers
      audio_server: DesktopAudioServer::new(),
      graphics_server: DesktopGraphicsServer::new(&window),
      input_server: DesktopInputServer::new(),

      // core
      event_loop,
      config,
      window,
    }
  }

  /// Sets the title of the platform's main window.
  pub fn set_title(&mut self, title: impl AsRef<str>) {
    self.window.set_title(title.as_ref());
  }
}

impl Platform for DesktopPlatform {
  fn run(&mut self, body: impl FnMut(&mut Self)) {
    use winit::platform::desktop::EventLoopExtDesktop;

    self.event_loop.run_return(|event, _, control_flow| {
      use winit::event::*;

      match event {
        Event::RedrawRequested(window_id) => unsafe {
          if window_id == self.window.id() {
            self.graphics_server.begin_frame();
            self.graphics_server.end_frame();
          }
        }
        Event::MainEventsCleared => {
          if self.config.update_continuously {
            self.window.request_redraw();
          }
        }
        Event::WindowEvent { window_id, event } if window_id == self.window.id() => {
          match event {
            WindowEvent::CloseRequested => {
              *control_flow = ControlFlow::Exit;
            }
            _ => {}
          }
        }
        _ => {}
      }
    });
  }
}
