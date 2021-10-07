//! A platform implementation for desktop PCs.

use std::collections::HashSet;

use raw_gl_context::{GlConfig, GlContext};
use winit::dpi::LogicalSize;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

use crate::graphics::{Buffer, Color, PrimitiveTopology, Viewport};
use crate::input::{Key, MouseButton};
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
  window: Window,
  context: GlContext,
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
    // prepare the main window and event loop
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(config.title)
        .with_inner_size(LogicalSize::new(config.size.0, config.size.1))
        .build(&event_loop)?;

    // prepare and load opengl functionality
    let context = GlContext::create(&window, GlConfig::default()).unwrap();
    context.make_current();
    gl::load_with(|symbol| context.get_proc_address(symbol) as *const _);

    Ok(Self {
      // core
      event_loop: Some(event_loop),
      window,
      context,
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
    use winit::platform::desktop::EventLoopExtDesktop;

    let mut event_loop = self.event_loop.take().unwrap();

    event_loop.run_return(move |event, _, control_flow| {
      use winit::event::*;

      match event {
        // generic winit events
        Event::RedrawRequested(window_id) => {
          if window_id == self.window.id() {
            self.context.make_current();

            callback(&mut self);

            self.context.swap_buffers();
            self.context.make_not_current();
          }
        }
        // generic window events
        Event::WindowEvent { window_id, event } if window_id == self.window.id() => match event {
          WindowEvent::CloseRequested => {
            *control_flow = ControlFlow::Exit;
          }
          _ => {}
        }
        _ => {}
      }
    });
  }
}

impl AudioDevice for DesktopPlatform {}

impl GraphicsDevice for DesktopPlatform {
  fn clear_color_buffer(&mut self, color: Color) {
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

  fn clear_depth_buffer(&mut self) {
    unsafe {
      gl::Clear(gl::DEPTH_BUFFER_BIT);
    }
  }

  fn set_viewport(&mut self, viewport: Viewport) {
    unsafe {
      gl::Viewport(0, 0, viewport.width as i32, viewport.height as i32);
    }
  }

  fn draw_mesh(&mut self, topology: PrimitiveTopology, vertex_buffer: &Buffer, index_buffer: &Buffer, vertex_count: usize) {
    unimplemented!()
  }
}

impl InputDevice for DesktopPlatform {
  fn is_button_up(&self, button: MouseButton) -> bool { !self.pressed_buttons.contains(&button) }
  fn is_button_down(&self, button: MouseButton) -> bool { self.pressed_buttons.contains(&button) }
  fn is_button_pressed(&self, button: MouseButton) -> bool { self.pressed_buttons.contains(&button) }

  fn is_key_up(&self, key: Key) -> bool { !self.pressed_keys.contains(&key) }
  fn is_key_down(&self, key: Key) -> bool { self.pressed_keys.contains(&key) }
  fn is_key_pressed(&self, key: Key) -> bool { self.pressed_keys.contains(&key) }
}

impl PlatformWindow for DesktopPlatform {
  fn set_title(&mut self, title: impl AsRef<str>) {
    self.window.set_title(title.as_ref());
  }
}

impl From<winit::event::MouseButton> for MouseButton {
  fn from(button: winit::event::MouseButton) -> Self {
    match button {
      winit::event::MouseButton::Left => Self::Left,
      winit::event::MouseButton::Right => Self::Right,
      winit::event::MouseButton::Middle => Self::Middle,
      winit::event::MouseButton::Other(_) => Self::Middle,
    }
  }
}

impl From<winit::event::ScanCode> for Key {
  fn from(code: u32) -> Self {
    Self::from_scan_code(code)
  }
}

impl From<winit::error::OsError> for Error {
  fn from(_: winit::error::OsError) -> Self {
    Error::General
  }
}
