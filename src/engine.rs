//! Game framework for Surreal.
//!
//! Bootstrapping and other framework systems for common projects.

pub use ecs::*;
pub use scenes::*;

mod ecs;
mod scenes;

use glutin::{window::Window, ContextBuilder};
use log::LevelFilter;
use glutin::{
  dpi::LogicalSize,
  event_loop::{ControlFlow, EventLoop},
  window::{Icon, WindowBuilder},
};

use crate::{
  assets::AssetManager,
  audio::{AudioServer, OpenALAudioBackend},
  diagnostics::ConsoleLogger,
  graphics::{GraphicsServer, ImageFormat, OpenGLGraphicsBackend},
  input::InputBackend,
  maths::vec2,
  utilities::{Clock, FrameCounter, IntervalTimer, TimeSpan},
};

// TODO: scene management
// TODO: plugin management (profiler, console, etc)
// TODO: better rendering pipeline support

/// Configuration for the `Engine`.
///
/// This struct defines how to set-up the game and initial settings.
#[derive(Clone, Debug)]
pub struct Configuration {
  pub title: &'static str,
  pub size: (u32, u32),
  pub vsync_enabled: bool,
  pub samples: u16,
  pub update_continuously: bool,
  pub run_in_background: bool,
  pub transparent_window: bool,
  pub show_fps_in_title: bool,
  pub log_level: LevelFilter,
  pub icon: Option<&'static [u8]>,
}

impl Default for Configuration {
  fn default() -> Self {
    Self {
      title: "Surreal",
      size: (1280, 720),
      vsync_enabled: true,
      samples: 0,
      update_continuously: true,
      run_in_background: false,
      transparent_window: false,
      show_fps_in_title: true,
      log_level: LevelFilter::Info,
      icon: Some(include_bytes!("../surreal.ico")),
    }
  }
}

/// Contains information on the game's timing state.
pub struct GameTime {
  pub delta_time: f32,
  pub total_time: f32,
}

/// The context for a single tick of the main loop.
pub struct GameTick {
  pub time: GameTime,
  is_exiting: bool,
}

impl GameTick {
  /// Exits the engine at the end of the current tick.
  pub fn exit(&mut self) {
    self.is_exiting = true;
  }
}

/// The core engine for Surreal.
///
/// This struct manages core systems and facilitates the main game loop.
pub struct Engine {
  // core systems
  pub audio: AudioServer,
  pub graphics: GraphicsServer,
  pub input: InputBackend,

  // window management
  config: Configuration,
  window: Window,
  cursor_icon: egui::CursorIcon,
  event_loop: Option<EventLoop<()>>,
  is_focused: bool,

  // timing
  clock: Clock,
  frame_timer: IntervalTimer,
  frame_counter: FrameCounter,
}

impl Engine {
  /// Starts the engine with the given configuration.
  pub fn start(configuration: Configuration, mut setup: impl FnMut(Engine, AssetManager)) {
    use crate::graphics::*;

    // set-up diagnostics
    ConsoleLogger::install(configuration.log_level);
    profiling::register_thread!("Main Thread");

    // set-up core engine
    log::trace!("Configuring engine");

    let engine = Engine::new(configuration);
    let graphics = &engine.graphics;

    // set-up asset manager
    log::trace!("Configuration asset manager");

    let mut assets = AssetManager::new();

    assets.add_loader(BitmapFontLoader {});
    assets.add_loader(ImageLoader { format: None });

    assets.add_loader(TextureLoader {
      server: graphics.clone(),
      options: TextureOptions::default(),
    });

    assets.add_loader(ShaderProgramLoader {
      server: graphics.clone(),
    });

    assets.add_loader(MaterialLoader {
      server: graphics.clone(),
    });

    log::trace!("Running engine setup");

    setup(engine, assets);
  }

  /// Creates a new engine, bootstrapping all core systems and opening the main display.
  pub fn new(config: Configuration) -> Self {
    // prepare the main window and event loop
    let event_loop = EventLoop::new();

    log::trace!("Building main window");

    let window = WindowBuilder::new()
      .with_title(config.title)
      .with_inner_size(LogicalSize::new(config.size.0, config.size.1))
      .with_resizable(true)
      .with_transparent(config.transparent_window)
      .with_window_icon(config.icon.map(|buffer| {
        let image = image::load_from_memory_with_format(buffer, ImageFormat::Ico)
          .expect("Failed to decode icon data");

        let rgba = image.as_rgba8().expect("Image was not in RGBA format");

        let pixels = rgba.pixels().flat_map(|pixel| pixel.0).collect();
        let width = rgba.width();
        let height = rgba.height();

        Icon::from_rgba(pixels, width, height).expect("Failed to convert icon from raw image")
      }));

    log::trace!("Building OpenGL context");

    // glutin tries to be safe via the type system, what a mess.
    let context = ContextBuilder::new()
      .with_vsync(config.vsync_enabled)
      .with_multisampling(config.samples)
      .build_windowed(window, &event_loop)
      .unwrap();

    // HACK: unpick the window from glutin so we can manage it ourselves
    let (context, window) = unsafe { context.make_current().unwrap().split() };

    let pixels_per_point = window.scale_factor() as f32;
    let audio = OpenALAudioBackend::new();
    let graphics = OpenGLGraphicsBackend::new(context);

    Self {
      // servers
      audio: AudioServer::new(Box::new(audio)),
      graphics: GraphicsServer::new(Box::new(graphics)),
      input: InputBackend::new(pixels_per_point),

      // window management
      config,
      window,
      cursor_icon: egui::CursorIcon::None,
      event_loop: Some(event_loop),
      is_focused: true,

      // timing
      clock: Clock::new(),
      frame_timer: IntervalTimer::new(TimeSpan::from_seconds(1.)),
      frame_counter: FrameCounter::new(32),
    }
  }

  /// Runs a variable step game loop against the engine.
  ///
  /// This method will block until the game is closed.
  pub fn run_variable_step(self, mut body: impl FnMut(&mut Engine, &mut GameTick)) {
    let mut timer = Clock::new();

    self.run(move |engine, control_flow| {
      // capture timing information
      let mut tick = GameTick {
        time: GameTime {
          delta_time: timer.tick(),
          total_time: timer.total_time(),
        },
        is_exiting: false,
      };

      // run main loop
      body(engine, &mut tick);

      if tick.is_exiting {
        *control_flow = ControlFlow::Exit;
      }

      profiling::finish_frame!();
    });
  }

  /// Runs the given delegate as the main loop for the engine.
  ///
  /// This method will block until the game is closed.
  pub fn run(mut self, mut body: impl FnMut(&mut Self, &mut ControlFlow)) {
    use glutin::event::*;
    use glutin::platform::run_return::EventLoopExtRunReturn;

    log::trace!("Entering main event loop");

    // use this hack to unpack the event loop out of 'self' and then remove the 'static
    // lifetime bound on run_return so that body can access things in self without lifetime woes.
    let mut event_loop = self.event_loop.take().unwrap();

    event_loop.run_return(move |event, _, control_flow| {
      match event {
        Event::RedrawRequested(window_id) => {
          if window_id == self.window.id() {
            // update graphics and run main loop
            self.graphics.begin_frame();
            {
              body(&mut self, control_flow);
            }
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
          if (self.config.update_continuously && self.is_focused) || self.config.run_in_background {
            *control_flow = ControlFlow::Poll;
            self.window.request_redraw();
          } else {
            *control_flow = ControlFlow::Wait;
            self.window.request_redraw();
          }
        }
        Event::WindowEvent { window_id, event } if window_id == self.window.id() => match event {
          WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
            self.input.pixels_per_point = scale_factor as f32;

            log::trace!("Window scale factor changed to {}", scale_factor);
          }
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
          WindowEvent::ReceivedCharacter(character) => {
            self.input.on_character_received(character);
          }
          WindowEvent::ModifiersChanged(modifiers) => {
            self.input.on_modifiers_changed(modifiers);
          }
          WindowEvent::Focused(focused) => {
            self.is_focused = focused;
            self.input.on_modifiers_changed(ModifiersState::default());

            if focused {
              log::trace!("Window gained focus");
            } else {
              log::trace!("Window lost focus");
            }
          }
          WindowEvent::Resized(size) => {
            let size = (size.width as usize, size.height as usize);

            self.graphics.set_viewport_size(size);

            log::trace!("Window resized to {}x{}", size.0, size.1);
          }
          WindowEvent::CloseRequested => {
            *control_flow = ControlFlow::Exit;
          }
          _ => {}
        },
        _ => {}
      }
    });

    log::trace!("Exiting main event loop")
  }
}

/// Allow the engine to be used in egui rendering.
impl crate::ui::UserInterfaceHost for Engine {
  fn pixels_per_point(&self) -> f32 {
    self.window.scale_factor() as f32
  }

  fn set_exclusive_keyboard_input(&mut self, exclusive: bool) {
    self.input.exclusive_keyboard_input = exclusive;
  }

  fn set_exclusive_pointer_input(&mut self, exclusive: bool) {
    self.input.exclusive_pointer_input = exclusive;
  }

  fn set_cursor_icon(&mut self, cursor_icon: egui::CursorIcon) {
    // prevent flickering near frame boundary when Windows OS tries to control cursor icon for window resizing
    if self.cursor_icon == cursor_icon {
      return;
    }

    self.cursor_icon = cursor_icon;

    if let Some(cursor_icon) = translate_cursor(cursor_icon) {
      // TODO: if cursor in window?
      self.window.set_cursor_visible(true);
      self.window.set_cursor_icon(cursor_icon);
    } else {
      self.window.set_cursor_visible(false);
    }
  }

  fn raw_input(&self) -> &egui::RawInput {
    &self.input.raw_input
  }

  fn request_redraw(&self) {
    self.window.request_redraw();
  }
}

/// Converts an egui cursor to a winit cursor.
fn translate_cursor(cursor_icon: egui::CursorIcon) -> Option<glutin::window::CursorIcon> {
  match cursor_icon {
    egui::CursorIcon::None => None,

    egui::CursorIcon::Alias => Some(glutin::window::CursorIcon::Alias),
    egui::CursorIcon::AllScroll => Some(glutin::window::CursorIcon::AllScroll),
    egui::CursorIcon::Cell => Some(glutin::window::CursorIcon::Cell),
    egui::CursorIcon::ContextMenu => Some(glutin::window::CursorIcon::ContextMenu),
    egui::CursorIcon::Copy => Some(glutin::window::CursorIcon::Copy),
    egui::CursorIcon::Crosshair => Some(glutin::window::CursorIcon::Crosshair),
    egui::CursorIcon::Default => Some(glutin::window::CursorIcon::Default),
    egui::CursorIcon::Grab => Some(glutin::window::CursorIcon::Grab),
    egui::CursorIcon::Grabbing => Some(glutin::window::CursorIcon::Grabbing),
    egui::CursorIcon::Help => Some(glutin::window::CursorIcon::Help),
    egui::CursorIcon::Move => Some(glutin::window::CursorIcon::Move),
    egui::CursorIcon::NoDrop => Some(glutin::window::CursorIcon::NoDrop),
    egui::CursorIcon::NotAllowed => Some(glutin::window::CursorIcon::NotAllowed),
    egui::CursorIcon::PointingHand => Some(glutin::window::CursorIcon::Hand),
    egui::CursorIcon::Progress => Some(glutin::window::CursorIcon::Progress),

    egui::CursorIcon::ResizeHorizontal => Some(glutin::window::CursorIcon::EwResize),
    egui::CursorIcon::ResizeNeSw => Some(glutin::window::CursorIcon::NeswResize),
    egui::CursorIcon::ResizeNwSe => Some(glutin::window::CursorIcon::NwseResize),
    egui::CursorIcon::ResizeVertical => Some(glutin::window::CursorIcon::NsResize),

    egui::CursorIcon::ResizeEast => Some(glutin::window::CursorIcon::EResize),
    egui::CursorIcon::ResizeSouthEast => Some(glutin::window::CursorIcon::SeResize),
    egui::CursorIcon::ResizeSouth => Some(glutin::window::CursorIcon::SResize),
    egui::CursorIcon::ResizeSouthWest => Some(glutin::window::CursorIcon::SwResize),
    egui::CursorIcon::ResizeWest => Some(glutin::window::CursorIcon::WResize),
    egui::CursorIcon::ResizeNorthWest => Some(glutin::window::CursorIcon::NwResize),
    egui::CursorIcon::ResizeNorth => Some(glutin::window::CursorIcon::NResize),
    egui::CursorIcon::ResizeNorthEast => Some(glutin::window::CursorIcon::NeResize),
    egui::CursorIcon::ResizeColumn => Some(glutin::window::CursorIcon::ColResize),
    egui::CursorIcon::ResizeRow => Some(glutin::window::CursorIcon::RowResize),

    egui::CursorIcon::Text => Some(glutin::window::CursorIcon::Text),
    egui::CursorIcon::VerticalText => Some(glutin::window::CursorIcon::VerticalText),
    egui::CursorIcon::Wait => Some(glutin::window::CursorIcon::Wait),
    egui::CursorIcon::ZoomIn => Some(glutin::window::CursorIcon::ZoomIn),
    egui::CursorIcon::ZoomOut => Some(glutin::window::CursorIcon::ZoomOut),
  }
}
