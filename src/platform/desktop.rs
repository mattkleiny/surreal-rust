//! A platform implementation for desktop PCs.

use std::collections::HashSet;

use winit::dpi::LogicalSize;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

use audio::DesktopAudioServer;
use graphics::DesktopGraphicsServer;
use input::DesktopInputServer;

use crate::audio::AudioHandle;
use crate::graphics::{Color, GraphicsContext, GraphicsHandle, Viewport};
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
  event_loop: Option<EventLoop<()>>,
  window: Window,
  is_exiting: bool,

  // servers
  pub audio: DesktopAudioServer,
  pub graphics: GraphicsContext,
  pub input: DesktopInputServer,
}

impl DesktopPlatform {
  pub fn new(config: Configuration) -> Self {
    // prepare the main window and event loop
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(config.title)
        .with_inner_size(LogicalSize::new(config.size.0, config.size.1))
        .with_resizable(true)
        .build(&event_loop)
        .unwrap();

    Self {
      // servers
      audio: DesktopAudioServer::new(),
      graphics: GraphicsContext::new(DesktopGraphicsServer::new(&window)),
      input: DesktopInputServer::new(),

      // core
      event_loop: Some(event_loop),
      config,
      window,
      is_exiting: false,
    }
  }

  /// Sets the title of the platform's main window.
  pub fn set_title(&mut self, title: impl AsRef<str>) {
    self.window.set_title(title.as_ref());
  }

  /// Exits the platform at the next loop.
  pub fn exit(&mut self) {
    self.is_exiting = true;
  }
}

impl Platform for DesktopPlatform {
  fn run(&mut self, mut body: impl FnMut(&mut Self)) {
    use winit::platform::desktop::EventLoopExtDesktop;

    let mut event_loop = self.event_loop.take().unwrap();

    event_loop.run_return(|event, _, control_flow| {
      use winit::event::*;

      match event {
        Event::RedrawRequested(window_id) => unsafe {
          if window_id == self.window.id() {
            self.graphics.begin_frame();
            body(self);
            self.graphics.end_frame();
          }
        }
        Event::MainEventsCleared => {
          if self.is_exiting {
            *control_flow = ControlFlow::Exit;
          } else if self.config.update_continuously {
            self.window.request_redraw();
          }
        }
        Event::WindowEvent { window_id, event } if window_id == self.window.id() => {
          match event {
            WindowEvent::MouseInput { button, state, .. } => {
              self.input.on_mouse_event(button, state);
            }
            WindowEvent::KeyboardInput { input, .. } => {
              self.input.on_keyboard_event(input);
            }
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
