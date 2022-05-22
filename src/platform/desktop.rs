//! A platform implementation for desktop PCs.

use image::ImageFormat;
use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;
use winit::window::{Icon, Window, WindowBuilder};

use audio::DesktopAudioServer;
use graphics::DesktopGraphicsServer;
use input::DesktopInputServer;

use crate::audio::AudioContext;
use crate::graphics::GraphicsContext;
use crate::platform::{Platform, PlatformHost};
use crate::utilities::{Clock, FrameCounter, IntervalTimer, TimeSpan};

mod audio;
mod graphics;
mod input;

/// Configuration for the `DesktopPlatform`.
#[derive(Clone, Debug)]
pub struct Configuration {
  pub title: &'static str,
  pub size: (u32, u32),
  pub update_continuously: bool,
  pub show_fps_in_title: bool,
  pub icon: Option<&'static [u8]>,
}

impl Default for Configuration {
  fn default() -> Self {
    Self {
      title: "Surreal",
      size: (1280, 720),
      update_continuously: true,
      show_fps_in_title: true,
      icon: Some(include_bytes!("../../surreal.ico")),
    }
  }
}

/// A `Platform` implementation for desktop PCs.
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
        let image = image::load_from_memory_with_format(buffer, ImageFormat::Ico).expect("Failed to decode icon data");
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
      audio: AudioContext::new(DesktopAudioServer::new()),
      graphics: GraphicsContext::new(DesktopGraphicsServer::new(&window)),
      input: DesktopInputServer::new(),

      // core
      window,
      event_loop: Some(event_loop),
      config: self.config.clone(),
      is_focused: false,
      is_closing: false,

      // timing
      clock: Clock::new(),
      frame_timer: IntervalTimer::new(TimeSpan::from_seconds(1.)),
      frame_counter: FrameCounter::new(32)
    }
  }
}

/// The host for the desktop platform.
pub struct DesktopPlatformHost {
  // servers
  pub audio: AudioContext,
  pub graphics: GraphicsContext,
  pub input: DesktopInputServer,

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
  /// Sets the title of the platform's main window.
  pub fn set_title(&mut self, title: impl AsRef<str>) {
    self.window.set_title(title.as_ref());
  }

  /// Exits the platform at the next tick.
  pub fn exit(&mut self) {
    self.is_closing = true;
  }
}

impl PlatformHost for DesktopPlatformHost {
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

  fn run(&mut self, mut body: impl FnMut(&mut Self)) {
    use winit::event_loop::ControlFlow;
    use winit::platform::desktop::EventLoopExtDesktop;

    let mut event_loop = self.event_loop.take().unwrap();

    event_loop.run_return(|event, _, control_flow| unsafe {
      use winit::event::*;

      match event {
        Event::RedrawRequested(window_id) => {
          if window_id == self.window.id() {
            self.graphics.begin_frame();
            body(self);
            self.graphics.end_frame();
          }
        }
        Event::MainEventsCleared => {
          // update the fps counter, if enabled
          if self.config.show_fps_in_title {
            let delta_time = self.clock.tick();

            self.frame_counter.tick(delta_time);

            if self.frame_timer.tick(delta_time) {
              self.window.set_title(&format!("{} - FPS: {:.2}", self.config.title, self.frame_counter.fps()));

              self.frame_timer.reset();
            }
          }

          // main control flow
          if self.is_closing {
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
            WindowEvent::Focused(focused) => {
              self.is_focused = focused;
            }
            _ => {}
          }
        }
        _ => {}
      }
    });
  }

  fn exit(&mut self) {
    self.is_closing = true;
  }
}
