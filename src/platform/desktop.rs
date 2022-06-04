//! A platform implementation for desktop PCs.

use image::ImageFormat;
use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;
use winit::window::{Icon, Window, WindowBuilder};

use audio::DesktopAudioBackend;
use graphics::DesktopGraphicsBackend;
use input::DesktopInput;

use crate::audio::AudioServer;
use crate::framework::EventListener;
use crate::graphics::GraphicsServer;
use crate::maths::vec2;
use crate::utilities::{Clock, FrameCounter, IntervalTimer, TimeSpan};

use super::*;

mod audio;
mod graphics;
mod input;

/// Configuration for the [`DesktopPlatform`].
#[derive(Clone, Debug)]
pub struct Configuration {
  pub title: &'static str,
  pub size: (u32, u32),
  pub vsync_enabled: bool,
  pub update_continuously: bool,
  pub run_in_background: bool,
  pub show_fps_in_title: bool,
  pub icon: Option<&'static [u8]>,
}

impl Default for Configuration {
  fn default() -> Self {
    Self {
      title: "Surreal",
      size: (1280, 720),
      vsync_enabled: true,
      update_continuously: true,
      run_in_background: false,
      show_fps_in_title: true,
      icon: Some(include_bytes!("../../surreal.ico")),
    }
  }
}

/// A [`Platform`] implementation for desktop PCs.
pub struct DesktopPlatform {
  config: Configuration,
}

impl DesktopPlatform {
  pub fn new(config: Configuration) -> Self {
    Self { config }
  }
}

impl Platform for DesktopPlatform {
  type Host = DesktopPlatformHost;

  fn create_host(&self) -> Self::Host {
    // prepare the main window and event loop
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
      .with_title(self.config.title)
      .with_inner_size(LogicalSize::new(self.config.size.0, self.config.size.1))
      .with_resizable(true)
      .with_window_icon(self.config.icon.map(|buffer| {
        let image = image::load_from_memory_with_format(buffer, ImageFormat::Ico)
          .expect("Failed to decode icon data");
        let rgba = image.as_rgba8().expect("Image was not in RGBA format");

        let pixels = rgba.pixels().flat_map(|pixel| pixel.0).collect();
        let width = rgba.width();
        let height = rgba.height();

        Icon::from_rgba(pixels, width, height).expect("Failed to convert icon from raw image")
      }))
      .build(&event_loop)
      .unwrap();

    Self::Host {
      // servers
      audio: AudioServer::new(Box::new(DesktopAudioBackend::new())),
      graphics: GraphicsServer::new(Box::new(DesktopGraphicsBackend::new(
        &window,
        self.config.vsync_enabled,
      ))),
      input: DesktopInput::new(),

      // core
      window,
      event_loop: Some(event_loop),
      config: self.config.clone(),
      is_focused: true,
      is_closing: false,

      // timing
      clock: Clock::new(),
      frame_timer: IntervalTimer::new(TimeSpan::from_seconds(1.)),
      frame_counter: FrameCounter::new(32),
    }
  }
}

/// The host for the desktop platform.
pub struct DesktopPlatformHost {
  // servers
  audio: AudioServer,
  graphics: GraphicsServer,
  pub input: DesktopInput,

  // core
  window: Window,
  event_loop: Option<EventLoop<()>>,
  config: Configuration,
  is_focused: bool,
  is_closing: bool,

  // timing
  clock: Clock,
  frame_timer: IntervalTimer,
  frame_counter: FrameCounter,
}

impl DesktopPlatformHost {
  /// Requests a re-paint of the window.
  pub fn request_repaint(&mut self) {
    self.window.request_redraw();
  }

  /// Sets the title of the platform's main window.
  pub fn set_title(&mut self, title: impl AsRef<str>) {
    self.window.set_title(title.as_ref());
  }
}

impl PlatformHost for DesktopPlatformHost {
  fn audio(&self) -> &AudioServer {
    &self.audio
  }

  fn graphics(&self) -> &GraphicsServer {
    &self.graphics
  }

  fn width(&self) -> usize {
    self.window.inner_size().width as usize
  }

  fn height(&self) -> usize {
    self.window.inner_size().height as usize
  }

  fn is_focused(&self) -> bool {
    self.is_focused
  }

  fn is_closing(&self) -> bool {
    self.is_closing
  }

  fn run(&mut self, mut main_loop: impl FnMut(&mut Self)) {
    use winit::event_loop::ControlFlow;
    use winit::platform::desktop::EventLoopExtDesktop;

    let mut event_loop = self.event_loop.take().unwrap();

    event_loop.run_return(|event, _, control_flow| {
      use winit::event::*;

      match event {
        Event::RedrawRequested(window_id) => {
          if window_id == self.window.id() {
            // update graphics and run main loop
            self.graphics.begin_frame();
            main_loop(self);
            self.graphics.end_frame();

            // update input devices
            self.input.tick();
          }
        }
        Event::MainEventsCleared => {
          // update the fps counter, if enabled
          if self.config.update_continuously && self.config.show_fps_in_title && self.is_focused {
            let delta_time = self.clock.tick();

            self.frame_counter.tick(delta_time);

            if self.frame_timer.tick(delta_time) {
              self.window.set_title(&format!(
                "{} - FPS: {:.2}",
                self.config.title,
                self.frame_counter.fps()
              ));

              self.frame_timer.reset();
            }
          } else {
            self.window.set_title(self.config.title);
          }

          // main control flow
          if self.is_closing {
            *control_flow = ControlFlow::Exit;
          } else if (self.config.update_continuously && self.is_focused)
            || self.config.run_in_background
          {
            *control_flow = ControlFlow::Poll;
            self.window.request_redraw();
          } else {
            *control_flow = ControlFlow::Wait;
            self.window.request_redraw();
          }
        }
        Event::WindowEvent { window_id, event } if window_id == self.window.id() => match event {
          WindowEvent::CursorMoved { position, .. } => {
            let size = self.window.inner_size();

            self.input.on_mouse_move(
              vec2(position.x as f32, position.y as f32),
              vec2(size.width as f32, size.height as f32),
            );
          }
          WindowEvent::MouseWheel { delta, .. } => {
            self.input.on_mouse_wheel(&delta);
          }
          WindowEvent::MouseInput { button, state, .. } => {
            self.input.on_mouse_button(button, state);
          }
          WindowEvent::KeyboardInput { input, .. } => {
            self.input.on_keyboard_event(&input);
          }
          WindowEvent::ModifiersChanged(modifiers) => {
            self.input.on_modifiers_changed(modifiers);
          }
          WindowEvent::Focused(focused) => {
            self.is_focused = focused;
            self.input.on_modifiers_changed(ModifiersState::default());
          }
          WindowEvent::Resized(size) => {
            let size = (size.width as usize, size.height as usize);

            self.graphics.set_viewport_size(size);
          }
          WindowEvent::CloseRequested => {
            *control_flow = ControlFlow::Exit;
          }
          _ => {}
        },
        _ => {}
      }
    });
  }

  fn pump(&mut self, mut listener: impl EventListener + 'static) {
    use crate::framework::*;
    use winit::event_loop::ControlFlow;
    use winit::platform::desktop::EventLoopExtDesktop;

    let mut event_loop = self.event_loop.take().unwrap();

    event_loop.run_return(|event, _, control_flow| {
      use winit::event::*;

      match event {
        Event::RedrawRequested(window_id) => {
          if window_id == self.window.id() {
            // update graphics
            self.graphics.begin_frame();
            listener.on_event(&PlatformRenderEvent());
            self.graphics.end_frame();
          }
        }
        Event::MainEventsCleared => {
          // update application logic
          listener.on_event(&PlatformTickEvent());

          // update input devices
          self.input.tick();

          // update the fps counter, if enabled
          if self.config.update_continuously && self.config.show_fps_in_title && self.is_focused {
            let delta_time = self.clock.tick();

            self.frame_counter.tick(delta_time);

            if self.frame_timer.tick(delta_time) {
              self.window.set_title(&format!(
                "{} - FPS: {:.2}",
                self.config.title,
                self.frame_counter.fps()
              ));

              self.frame_timer.reset();
            }
          } else {
            self.window.set_title(self.config.title);
          }

          // main control flow
          if self.is_closing {
            *control_flow = ControlFlow::Exit;
          } else if (self.config.update_continuously && self.is_focused)
            || self.config.run_in_background
          {
            *control_flow = ControlFlow::Poll;
            self.window.request_redraw();
          } else {
            *control_flow = ControlFlow::Wait;
            self.window.request_redraw();
          }
        }
        Event::WindowEvent { window_id, event } if window_id == self.window.id() => match event {
          WindowEvent::CursorMoved { position, .. } => {
            let size = self.window.inner_size();

            let position = vec2(position.x as f32, position.y as f32);
            let size = vec2(size.width as f32, size.height as f32);

            self.input.on_mouse_move(position, size);

            listener.on_event(&MouseMoveEvent(position));
          }
          WindowEvent::MouseWheel { delta, .. } => {
            self.input.on_mouse_wheel(&delta);

            let mut delta = match delta {
              MouseScrollDelta::LineDelta(x, y) => {
                let points_per_scroll_line = 50.0;

                vec2(x, y) * points_per_scroll_line
              }
              MouseScrollDelta::PixelDelta(delta) => vec2(delta.x as f32, delta.y as f32),
            };

            delta.x *= -1.0;

            listener.on_event(&MouseScrollEvent(delta));
          }
          WindowEvent::MouseInput { button, state, .. } => {
            self.input.on_mouse_button(button, state);

            match state {
              ElementState::Pressed => listener.on_event(&MousePressEvent(button)),
              ElementState::Released => listener.on_event(&MouseReleaseEvent(button)),
            }
          }
          WindowEvent::KeyboardInput { input, .. } => {
            self.input.on_keyboard_event(&input);

            if let Some(key) = input.virtual_keycode {
              match input.state {
                ElementState::Pressed => listener.on_event(&KeyPressEvent(key)),
                ElementState::Released => listener.on_event(&KeyReleaseEvent(key)),
              };
            }
          }
          WindowEvent::ModifiersChanged(modifiers) => {
            self.input.on_modifiers_changed(modifiers);
          }
          WindowEvent::Focused(focused) => {
            self.is_focused = focused;
            self.input.on_modifiers_changed(ModifiersState::default());

            listener.on_event(&PlatformFocusEvent(focused));
          }
          WindowEvent::Resized(size) => {
            let size = (size.width as usize, size.height as usize);

            self.graphics.set_viewport_size(size);

            listener.on_event(&PlatformResizeEvent(size.0, size.1));
          }
          WindowEvent::CloseRequested => {
            *control_flow = ControlFlow::Exit;

            listener.on_event(&PlatformCloseEvent());
          }
          _ => {}
        },
        _ => {}
      }
    });
  }

  fn exit(&mut self) {
    self.is_closing = true;
  }
}
