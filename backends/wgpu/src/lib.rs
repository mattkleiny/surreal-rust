//! A winit/wgpu-based backend for Surreal.

use std::error::Error;

use ::graphics::graphics;
use common::uvec2;
use winit::{
  application::ApplicationHandler,
  dpi::PhysicalSize,
  event::WindowEvent,
  event_loop::{ActiveEventLoop, EventLoop},
  window::{Window, WindowAttributes, WindowId},
};

mod graphics;
mod input;

/// Settings for a window.
pub struct WindowSettings {
  pub title: String,
  pub width: u32,
  pub height: u32,
  pub resizable: bool,
}

/// Entry point for a winit-based application.
pub struct Application {
  settings: WindowSettings,
  window: Option<Window>,
  keyboard: input::WinitKeyboardDevice,
  mouse: input::WinitMouseDevice,
  callback: Box<dyn FnMut()>,
}

impl Default for WindowSettings {
  fn default() -> Self {
    Self {
      title: "Surreal".to_string(),
      width: 800,
      height: 600,
      resizable: true,
    }
  }
}

impl Application {
  /// Creates a new [`Application`] with the given window settings.
  pub fn start(settings: WindowSettings, callback: impl FnMut() + 'static) -> Result<(), Box<dyn Error>> {
    let mut application = Self {
      settings,
      window: None,
      keyboard: input::WinitKeyboardDevice::default(),
      mouse: input::WinitMouseDevice::default(),
      callback: Box::new(callback),
    };

    let event_loop = EventLoop::new()?;

    event_loop.run_app(&mut application)?;

    Ok(())
  }
}

impl ApplicationHandler for Application {
  fn resumed(&mut self, event_loop: &ActiveEventLoop) {
    use common::BlockableFuture;

    let attributes = WindowAttributes::default()
      .with_title(self.settings.title.clone())
      .with_resizable(self.settings.resizable)
      .with_inner_size(PhysicalSize::new(self.settings.width, self.settings.height));

    let window = event_loop.create_window(attributes).unwrap();
    let graphics = graphics::WgpuGraphicsBackend::new(&window).block().unwrap();

    ::graphics::GraphicsServer::install(graphics);

    window.request_redraw(); // request initial paint

    self.window = Some(window);
  }

  fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
    match event {
      WindowEvent::RedrawRequested => {
        self.keyboard.clear_events();
        self.mouse.clear_events();

        (self.callback)();

        if let Some(window) = self.window.as_ref() {
          window.request_redraw();
        }
      }
      WindowEvent::Resized(new_size) => {
        graphics().set_viewport_size(uvec2(new_size.width, new_size.height));
      }
      WindowEvent::KeyboardInput { event, .. } => {
        self.keyboard.handle_input(&event);
      }
      WindowEvent::MouseInput { state, button, .. } => {
        self.mouse.handle_input(state, button);
      }
      WindowEvent::CloseRequested => {
        event_loop.exit();
      }
      _ => {}
    }
  }
}
