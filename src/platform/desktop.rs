//! A platform implementation for desktop PCs.

use std::collections::HashSet;

use glutin::{
  ContextWrapper,
  dpi::LogicalSize,
  event::{ElementState, Event, KeyboardInput, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  platform::desktop::EventLoopExtDesktop,
  PossiblyCurrent,
  window::{Window, WindowBuilder},
};

use crate::audio::*;
use crate::graphics::*;
use crate::input::*;
use crate::maths::{vec2, Vector2};
use crate::platform::{*, Error as Error};

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
  type AudioDevice = Self;
  type GraphicsDevice = Self;
  type InputDevice = Self;
  type PlatformWindow = Self;

  fn audio(&mut self) -> &mut Self::AudioDevice { self }
  fn graphics(&mut self) -> &mut Self::GraphicsDevice { self }
  fn input(&mut self) -> &mut Self::InputDevice { self }
  fn window(&mut self) -> &mut Self::PlatformWindow { self }

  fn run(mut self, mut callback: impl FnMut(&mut Self)) {
    let mut event_loop = self.event_loop.take().unwrap();

    event_loop.run_return(move |event, _, control_flow| {
      match event {
        // generic winit events
        Event::MainEventsCleared => {
          callback(&mut self);

          if self.is_continuous_rendering {
            self.window_context.window().request_redraw();
          }
        }
        Event::RedrawRequested(window_id) => {
          if window_id == self.window_context.window().id() {
            self.window_context.swap_buffers().unwrap();
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

impl AudioDevice for DesktopPlatform {}

impl GraphicsDevice for DesktopPlatform {
  fn clear_active_frame_buffer(&mut self, color: Color) {
    unsafe {
      gl::ClearColor(
        color.r as f32 / 255.0,
        color.g as f32 / 255.0,
        color.b as f32 / 255.0,
        color.a as f32 / 255.0,
      );
      gl::Clear(gl::COLOR_BUFFER_BIT);
    }
  }

  fn set_viewport(&mut self, viewport: Viewport) {
    unsafe {
      gl::Viewport(0, 0, viewport.width as i32, viewport.height as i32);
    }
  }
}

impl InputDevice for DesktopPlatform {
  fn is_button_up(&self, button: MouseButton) -> bool {
    !self.pressed_buttons.contains(&button)
  }

  fn is_button_down(&self, button: MouseButton) -> bool {
    self.pressed_buttons.contains(&button)
  }

  fn is_button_pressed(&self, button: MouseButton) -> bool {
    self.pressed_buttons.contains(&button)
  }

  fn is_key_up(&self, key: Key) -> bool {
    !self.pressed_keys.contains(&key)
  }

  fn is_key_down(&self, key: Key) -> bool {
    self.pressed_keys.contains(&key)
  }

  fn is_key_pressed(&self, key: Key) -> bool {
    self.pressed_keys.contains(&key)
  }

  fn get_active_touches(&self) -> &[Touch] {
    unimplemented!()
  }
}

impl PlatformWindow for DesktopPlatform {
  fn set_title(&mut self, title: impl AsRef<str>) {
    self.window_context.window().set_title(title.as_ref());
  }
}

impl From<glutin::event::MouseButton> for MouseButton {
  fn from(button: glutin::event::MouseButton) -> Self {
    match button {
      glutin::event::MouseButton::Left => Self::Left,
      glutin::event::MouseButton::Right => Self::Right,
      glutin::event::MouseButton::Middle => Self::Middle,
      glutin::event::MouseButton::Other(_) => Self::Middle,
    }
  }
}

impl From<glutin::event::ScanCode> for Key {
  fn from(code: u32) -> Self {
    Self::from_scan_code(code)
  }
}

impl From<glutin::error::OsError> for Error {
  fn from(_: glutin::error::OsError) -> Self {
    Error::General
  }
}

impl From<glutin::CreationError> for Error {
  fn from(_: glutin::CreationError) -> Self {
    Error::General
  }
}
