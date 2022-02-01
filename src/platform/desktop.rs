//! A platform implementation for desktop PCs.

use std::collections::HashSet;

use raw_gl_context::{GlConfig, GlContext};
use winit::dpi::LogicalSize;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

use crate::audio::AudioHandle;
use crate::graphics::{Color, GraphicsHandle, Viewport};
use crate::input::{InputServer, Key, KeyboardDevice, MouseButton, MouseDevice};
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

  // servers
  audio_server: Rc<dyn AudioServer>,
  graphics_server: Rc<dyn GraphicsServer>,
  input_server: Rc<DesktopInputServer>,
}

impl DesktopPlatform {
  pub fn new(config: Configuration) -> PlatformResult<Self> {
    // prepare the main window and event loop
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(config.title)
        .with_inner_size(LogicalSize::new(config.size.0, config.size.1))
        .build(&event_loop)?;

    let audio_server = DesktopAudioServer::new();
    let graphics_server = DesktopGraphicsServer::new(&window);
    let input_server = DesktopInputServer::new();

    Ok(Self {
      // core
      event_loop: Some(event_loop),
      window,

      // servers
      audio_server: Rc::new(audio_server),
      graphics_server: Rc::new(graphics_server),
      input_server: Rc::new(input_server),
    })
  }

  /// Sets the title of the platform's main window.
  pub fn set_title(&mut self, title: impl AsRef<str>) {
    self.window.set_title(title.as_ref());
  }
}

impl Platform for DesktopPlatform {
  fn audio(&self) -> &Rc<dyn AudioServer> {
    &self.audio_server
  }

  fn graphics(&self) -> &Rc<dyn GraphicsServer> {
    &self.graphics_server
  }

  fn run(mut self) {
    use winit::platform::desktop::EventLoopExtDesktop;
    let mut event_loop = self.event_loop.take().unwrap();

    event_loop.run_return(move |event, _, control_flow| {
      use winit::event::*;

      match event {
        // generic winit events
        Event::RedrawRequested(window_id) => {
          if window_id == self.window.id() {
            let graphics_server = &self.graphics_server;

            graphics_server.begin_frame();
            graphics_server.end_frame();
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

/// The audio server for the desktop platform.
struct DesktopAudioServer {}

impl DesktopAudioServer {
  pub fn new() -> Self {
    Self {}
  }
}

unsafe impl AudioServer for DesktopAudioServer {
  fn create_clip(&self) -> AudioHandle {
    todo!()
  }

  fn upload_clip_data(&self, handle: AudioHandle, data: &[u8]) {
    todo!()
  }

  fn delete_clip(&self, handle: AudioHandle) {
    todo!()
  }
}

/// The graphics server for the desktop platform.
struct DesktopGraphicsServer {
  context: GlContext,
  is_continuous_rendering: bool,
}

impl DesktopGraphicsServer {
  pub fn new(window: &Window) -> Self {
    // prepare and load opengl functionality
    let context = GlContext::create(window, GlConfig::default()).unwrap();
    context.make_current();
    gl::load_with(|symbol| context.get_proc_address(symbol) as *const _);

    Self {
      context,
      is_continuous_rendering: true,
    }
  }
}

unsafe impl GraphicsServer for DesktopGraphicsServer {
  fn begin_frame(&self) {
    self.context.make_current();
  }

  fn end_frame(&self) {
    self.context.swap_buffers();
    self.context.make_not_current();
  }

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
    unsafe {
      gl::Clear(gl::DEPTH_BUFFER_BIT);
    }
  }

  fn flush_commands(&self) {
    unsafe {
      gl::Flush();
    }
  }

  fn create_buffer(&self) -> GraphicsHandle {
    unsafe {
      let mut id: u32 = 0;
      gl::GenBuffers(1, &mut id);
      GraphicsHandle(id)
    }
  }

  fn write_buffer_data(&self, buffer: GraphicsHandle, data: &[u8]) {
    todo!()
  }

  fn delete_buffer(&self, buffer: GraphicsHandle) {
    unsafe {
      gl::DeleteBuffers(1, &buffer.0);
    }
  }

  fn create_texture(&self) -> GraphicsHandle {
    unsafe {
      let mut id: u32 = 0;
      gl::GenTextures(1, &mut id);
      GraphicsHandle(id)
    }
  }

  fn write_texture_data(&self, texture: GraphicsHandle, data: &[u8]) {
    todo!()
  }

  fn delete_texture(&self, texture: GraphicsHandle) {
    unsafe {
      gl::DeleteTextures(1, &texture.0);
    }
  }

  fn create_shader(&self) -> GraphicsHandle {
    unsafe {
      GraphicsHandle(gl::CreateProgram())
    }
  }

  fn delete_shader(&self, shader: GraphicsHandle) {
    unsafe {
      gl::DeleteProgram(shader.0);
    }
  }
}

/// The server for input management.
struct DesktopInputServer {
  mouse_position: Vector2<f64>,
  mouse_delta: Vector2<f64>,
  pressed_buttons: HashSet<MouseButton>,
  released_buttons: HashSet<MouseButton>,
  pressed_keys: HashSet<Key>,
  released_keys: HashSet<Key>,
}

impl DesktopInputServer {
  pub fn new() -> Self {
    Self {
      mouse_position: vec2(0., 0.),
      mouse_delta: vec2(0., 0.),
      pressed_buttons: HashSet::new(),
      released_buttons: HashSet::new(),
      pressed_keys: HashSet::new(),
      released_keys: HashSet::new(),
    }
  }
}

unsafe impl InputServer for DesktopInputServer {
  fn keyboard_devices(&self) -> &[&dyn KeyboardDevice] {
    todo!()
  }

  fn mouse_devices(&self) -> &[&dyn MouseDevice] {
    todo!()
  }
}

impl KeyboardDevice for DesktopInputServer {
  fn is_key_up(&self, key: Key) -> bool {
    todo!()
  }

  fn is_key_down(&self, key: Key) -> bool {
    todo!()
  }

  fn is_key_pressed(&self, key: Key) -> bool {
    todo!()
  }
}

impl MouseDevice for DesktopInputServer {
  fn is_button_up(&self, button: MouseButton) -> bool {
    todo!()
  }

  fn is_button_down(&self, button: MouseButton) -> bool {
    todo!()
  }

  fn is_button_pressed(&self, button: MouseButton) -> bool {
    todo!()
  }
}