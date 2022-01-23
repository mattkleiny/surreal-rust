//! A platform implementation for desktop PCs.

use std::collections::HashSet;

use raw_gl_context::{GlConfig, GlContext};
use winit::dpi::LogicalSize;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

use crate::audio::AudioHandle;
use crate::graphics::{Color, GraphicsHandle, Viewport};
use crate::input::{Key, MouseButton};
use crate::maths::{vec2, Vector2};

use super::*;

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
  pub fn new(config: Configuration) -> PlatformResult<Self> {
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

  /// Sets the title of the platform's main window.
  pub fn set_title(&mut self, title: impl AsRef<str>) {
    self.window.set_title(title.as_ref());
  }
}

impl Platform for DesktopPlatform {
  type AudioServer = Self;
  type GraphicsServer = Self;

  fn audio(&mut self) -> &mut Self::AudioServer { self }
  fn graphics(&mut self) -> &mut Self::GraphicsServer { self }

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

unsafe impl AudioServer for DesktopPlatform {
  fn create_clip(&self) -> AudioHandle {
    todo!()
  }

  fn upload_clip_data<T>(&self, handle: AudioHandle, data: &[T]) {
    todo!()
  }

  fn delete_clip(&self, handle: AudioHandle) {
    todo!()
  }
}

unsafe impl GraphicsServer for DesktopPlatform {
  fn set_viewport(&self, viewport: Viewport) {
    unsafe {
      gl::Viewport(0, 0, viewport.width as i32, viewport.height as i32);
    }
  }

  fn clear_color_buffer(&self, color: Color) {
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

  fn clear_depth_buffer(&self) {
    todo!()
  }

  fn flush_commands(&self) {
    todo!()
  }

  fn create_buffer(&self) -> GraphicsHandle {
    todo!()
  }

  fn write_buffer_data<T>(&self, handle: GraphicsHandle, data: &[T]) {
    todo!()
  }

  fn delete_buffer(&self, handle: GraphicsHandle) {
    todo!()
  }

  fn create_texture(&self) -> GraphicsHandle {
    todo!()
  }

  fn write_texture_data<T>(&self, texture: GraphicsHandle, data: &[T]) {
    todo!()
  }

  fn delete_texture(&self) {
    todo!()
  }

  fn create_shader(&self) -> GraphicsHandle {
    todo!()
  }

  fn delete_shader(&self, shader: GraphicsHandle) {
    todo!()
  }
}