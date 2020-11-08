//! A platform implementation for desktop PCs.

use std::collections::HashSet;

use glutin::{ContextWrapper, PossiblyCurrent};
use winit::{
  dpi::LogicalSize,
  event::{ElementState, Event, KeyboardInput, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  platform::desktop::EventLoopExtDesktop,
  window::{Window, WindowBuilder},
};

use crate::input::{Key, MouseButton};
use crate::maths::{vec2, Vector2};
use crate::platform::{Platform, Error};

mod audio;
mod graphics;
mod input;
mod window;

/// Configuration for the `DesktopPlatform`.
#[derive(Copy, Clone, Debug)]
pub struct Configuration {
  pub title: &'static str,
  pub size: (u32, u32),
}

/// A `Platform` implementation for desktop PCs.
pub struct DesktopPlatform {
  // core
  event_loop: Option<EventLoop<()>>,
  window_context: ContextWrapper<PossiblyCurrent, Window>,
  is_continuous_rendering: bool,

  // input
  mouse_position: Vector2<f64>,
  mouse_delta: Vector2<f64>,
  pressed_buttons: HashSet<MouseButton>,
  released_buttons: HashSet<MouseButton>,
  pressed_keys: HashSet<Key>,
  released_keys: HashSet<Key>,
}

impl DesktopPlatform {
  pub fn new(config: Configuration) -> Result<Self, Error> {
    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new()
      .with_title(config.title)
      .with_inner_size(LogicalSize::new(config.size.0, config.size.1));

    // prepare the OpenGL window context
    let window_context = unsafe {
      glutin::ContextBuilder::new()
        .build_windowed(window_builder, &event_loop)?
        .make_current()
        .unwrap()
    };

    // load OpenGL functions from the associated binary
    gl::load_with(|ptr| window_context.get_proc_address(ptr) as *const _);

    Ok(Self {
      // core
      event_loop: Some(event_loop),
      window_context,
      is_continuous_rendering: true,

      // input
      mouse_position: vec2(0., 0.),
      mouse_delta: vec2(0., 0.),
      pressed_buttons: HashSet::new(),
      released_buttons: HashSet::new(),
      pressed_keys: HashSet::new(),
      released_keys: HashSet::new(),
    })
  }
}

impl Platform for DesktopPlatform {
  type Audio = Self;
  type Graphics = Self;
  type Input = Self;
  type Window = Self;

  fn audio(&mut self) -> &mut Self::Audio {
    self
  }

  fn graphics(&mut self) -> &mut Self::Graphics {
    self
  }

  fn input(&mut self) -> &mut Self::Input {
    self
  }

  fn window(&mut self) -> &mut Self::Window {
    self
  }

  fn run(mut self, mut callback: impl FnMut(&mut Self)) {
    let mut event_loop = self.event_loop.take().unwrap();

    event_loop.run_return(move |event, _, control_flow| {
      match event {
        // generic winit events
        Event::RedrawRequested(window_id) => {
          if window_id == self.window_context.window().id() {
            callback(&mut self);

            self.window_context.swap_buffers().unwrap();

            if self.is_continuous_rendering {
              self.window_context.window().request_redraw();
            }
          }
        }
        Event::Suspended => {}
        Event::Resumed => {}
        Event::LoopDestroyed => {}

        // generic window events
        Event::WindowEvent { window_id, event }
          if window_id == self.window_context.window().id() =>
        {
          match event {
            WindowEvent::Resized(new_size) => {
              self.window_context.resize(new_size);
            }
            WindowEvent::CloseRequested => {
              *control_flow = ControlFlow::Exit;
            }
            WindowEvent::CursorMoved { position, .. } => {
              self.mouse_delta = vec2(
                position.x - self.mouse_position.x,
                position.y - self.mouse_position.y,
              );
              self.mouse_position = vec2(position.x, position.y);
            }
            WindowEvent::MouseInput { button, state, .. } => {
              let button = button.into();

              if state == ElementState::Pressed {
                self.released_buttons.remove(&button);
                self.pressed_buttons.insert(button);
              } else {
                self.pressed_buttons.remove(&button);
                self.released_buttons.insert(button);
              }
            }
            WindowEvent::KeyboardInput {
              input: KeyboardInput {
                scancode, state, ..
              },
              ..
            } => {
              let key = scancode.into();

              if state == ElementState::Pressed {
                self.released_keys.remove(&key);
                self.pressed_keys.insert(key);
              } else {
                self.pressed_keys.remove(&key);
                self.released_keys.insert(key);
              }
            }
            WindowEvent::Focused(is_focused) => {}
            _ => {}
          }
        }
        _ => {}
      }
    });
  }
}

impl From<winit::error::OsError> for Error {
  fn from(_: winit::error::OsError) -> Self {
    Error::General
  }
}

impl From<glutin::CreationError> for Error {
  fn from(_: glutin::CreationError) -> Self {
    Error::General
  }
}
