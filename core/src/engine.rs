//! Game framework for Surreal.
//!
//! Bootstrapping and other framework systems for common projects.

use std::time::Duration;

use glutin::{
  dpi::LogicalSize,
  event::WindowEvent,
  event_loop::{ControlFlow, EventLoop},
  window::{Icon, Window, WindowBuilder},
  ContextBuilder,
};
use log::LevelFilter;
use winit::event_loop::EventLoopProxy;

use crate::{
  assets::AssetManager,
  audio::{AudioServer, OpenALAudioBackend},
  diagnostics::{profiling, ConsoleLoggerBuilder},
  graphics::{GraphicsServer, ImageFormat, OpenGLGraphicsBackend, Renderer},
  input,
  input::InputBackend,
  maths::{uvec2, vec2},
  scene::{SceneEvent, SceneGraph},
  utilities::{DeltaClock, FrameCounter, IntervalTimer, TimeSpan},
};

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
  pub is_window_visible: bool,
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
      is_window_visible: true,
      transparent_window: false,
      show_fps_in_title: true,
      log_level: LevelFilter::Info,
      icon: Some(include_bytes!("../../surreal.ico")),
    }
  }
}

/// Contains information on the game's timing state.
#[derive(Copy, Clone, Debug)]
pub struct GameTime {
  pub delta_time: f32,
  pub total_time: f32,
}

/// Different kinds of events that can be dispatched to an application.
pub enum TickEvent<'a> {
  /// Indicating the application should update.
  Update(GameTime),
  /// Indicating the application should draw.
  Draw(GameTime),
  /// An event from the underlying window platform.
  Window(&'a WindowEvent<'a>),
}

/// A response for a tick in the application.
pub enum TickResponse {
  /// The application should continue.
  Continue,
  /// The application should stop.
  Exit,
}

/// Represents an application that can be used in an [`Engine`].
#[allow(unused_variables)]
pub trait Application: Sized {
  /// Builds the [`Application`] instance.
  fn new(engine: &Engine, assets: &AssetManager) -> crate::Result<Self>;

  /// Called when the application is to be updated.
  fn on_update(&mut self, engine: &mut Engine, time: GameTime) {}

  /// Called when the application is to be drawn.
  fn on_draw(&mut self, engine: &mut Engine, time: GameTime) {}

  /// Invoked when a [`WindowEvent`] is received.
  fn on_window_event(&mut self, engine: &mut Engine, event: &WindowEvent) {}

  /// Notifies the application of an [`TickEvent`]. l
  fn notify(&mut self, engine: &mut Engine, event: TickEvent) -> TickResponse {
    match event {
      TickEvent::Update(time) => self.on_update(engine, time),
      TickEvent::Draw(time) => self.on_draw(engine, time),
      TickEvent::Window(event) => self.on_window_event(engine, event),
    }

    TickResponse::Continue
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
  event_loop_proxy: EventLoopProxy<()>,
  event_loop: Option<EventLoop<()>>,
  is_focused: bool,
  repaint_after: Option<Duration>,

  // timing
  clock: DeltaClock,
  frame_timer: IntervalTimer,
  frame_counter: FrameCounter,
}

impl Engine {
  /// Creates a new engine, bootstrapping all core systems and opening the main display.
  pub fn new(config: Configuration) -> Self {
    #[cfg(target_os = "windows")]
    fn build_event_loop() -> EventLoop<()> {
      use winit::platform::windows::EventLoopExtWindows;
      EventLoop::new_any_thread()
    }

    #[cfg(target_os = "linux")]
    fn build_event_loop() -> EventLoop<()> {
      use winit::platform::unix::EventLoopExtUnix;
      EventLoop::new_any_thread()
    }

    #[cfg(target_os = "macos")]
    fn build_event_loop() -> EventLoop<()> {
      EventLoop::new()
    }

    // prepare the main window and event loop
    let event_loop = build_event_loop();

    log::trace!("Building main window");

    let window = WindowBuilder::new()
      .with_title(config.title)
      .with_inner_size(LogicalSize::new(config.size.0, config.size.1))
      .with_resizable(true)
      .with_transparent(config.transparent_window)
      .with_visible(config.is_window_visible)
      .with_window_icon(config.icon.map(|buffer| {
        let image = image::load_from_memory_with_format(buffer, ImageFormat::Ico).expect("Failed to decode icon data");
        let rgba = image.as_rgba8().expect("Image was not in RGBA format");

        let pixels = rgba.pixels().flat_map(|pixel| pixel.0).collect();
        let width = rgba.width();
        let height = rgba.height();

        Icon::from_rgba(pixels, width, height).expect("Failed to convert icon from raw image")
      }));

    // glutin tries to be safe via the type system
    let context = ContextBuilder::new()
      .with_vsync(config.vsync_enabled)
      .with_multisampling(config.samples)
      .build_windowed(window, &event_loop)
      .expect("Failed to build main window context");

    // unpick the window from glutin so we can manage it ourselves
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
      event_loop_proxy: event_loop.create_proxy(),
      event_loop: Some(event_loop),
      is_focused: true,
      repaint_after: None,

      // timing
      clock: DeltaClock::new(),
      frame_timer: IntervalTimer::new(TimeSpan::from_seconds(1.)),
      frame_counter: FrameCounter::new(32),
    }
  }

  /// Starts the engine with the given configuration.
  pub fn start(configuration: Configuration, setup: impl FnOnce(Engine, AssetManager)) {
    use crate::graphics::*;

    // set-up diagnostics
    ConsoleLoggerBuilder::new().with_level(configuration.log_level).install();
    profiling::enable_profiling();

    // set-up core engine
    log::trace!("Starting engine");

    let engine = Engine::new(configuration);
    let graphics = &engine.graphics;

    // set-up asset manager
    log::trace!("Building asset manager");

    let mut assets = AssetManager::new();

    assets.add_loader(BitmapFontLoader {});
    assets.add_loader(VectorFontLoader {
      graphics: graphics.clone(),
      font_size: 16.,
      atlas_stride: 16,
      atlas_cell_size: uvec2(16, 16),
    });

    assets.add_loader(ImageLoader { format: None });

    assets.add_loader(TextureLoader {
      graphics: graphics.clone(),
      options: TextureOptions::default(),
    });

    assets.add_loader(ShaderProgramLoader {
      graphics: graphics.clone(),
    });

    assets.add_loader(MaterialLoader {
      graphics: graphics.clone(),
    });

    assets.add_loader(ColorPaletteLoader::<Color>::default());
    assets.add_loader(ColorPaletteLoader::<Color32>::default());

    log::trace!("Running engine setup");

    setup(engine, assets);
  }

  /// Builds a [`Engine`] that runs the given [`SceneGraph`].
  pub fn from_scene(configuration: Configuration, setup: impl Fn(&Engine, &AssetManager) -> SceneGraph) {
    Engine::start(configuration, |engine, assets| {
      let mut scene_graph = setup(&engine, &assets);
      let mut renderer = Renderer::new(&engine.graphics);

      engine.run_variable_step(|_, time| {
        renderer.begin_frame();

        scene_graph.notify(SceneEvent::Update(time.delta_time));
        scene_graph.notify(SceneEvent::Render(&mut renderer));

        renderer.end_frame();

        TickResponse::Continue
      });
    });
  }

  /// Builds a [`Engine`] that runs the given [`Application`].  
  pub fn from_application<A: Application>(configuration: Configuration) {
    Engine::start(configuration, |engine, assets| {
      let mut application = A::new(&engine, &assets).expect("Failed to create application");

      engine.run(|engine, event| application.notify(engine, event));
    })
  }

  /// Runs a variable step game loop against the engine.
  ///
  /// This method will block until the game is closed.
  pub fn run_variable_step(self, mut body: impl FnMut(&mut Engine, GameTime) -> TickResponse) {
    self.run(move |engine, event| match event {
      TickEvent::Update(time) => body(engine, time),
      _ => TickResponse::Continue,
    });
  }

  /// Runs the given delegate as the main loop for the engine.
  ///
  /// This method will block until the game is closed.
  pub fn run(mut self, mut body: impl FnMut(&mut Self, TickEvent) -> TickResponse) {
    use glutin::event::*;
    use glutin::platform::run_return::EventLoopExtRunReturn;

    log::trace!("Entering main event loop");

    // use this hack to unpack the event loop out of 'self' and then remove the 'static
    // lifetime bound on run_return so that body can access things in self without lifetime woes.
    let mut event_loop = self
      .event_loop
      .take()
      .expect("The engine has already run once and cannot be started again");

    event_loop.run_return(move |event, _, control_flow| {
      match event {
        Event::RedrawRequested(window_id) => {
          if window_id == self.window.id() {
            // update graphics and run draw loop
            self.graphics.begin_frame();

            let time = GameTime {
              delta_time: self.clock.last_delta_time(),
              total_time: self.clock.total_time(),
            };

            body(&mut self, TickEvent::Draw(time));
            self.input.tick(self.clock.total_time());

            self.graphics.end_frame();
          }
        }
        Event::MainEventsCleared => {
          self.clock.tick();

          let time = GameTime {
            delta_time: self.clock.last_delta_time(),
            total_time: self.clock.total_time(),
          };

          // update core systems
          match body(&mut self, TickEvent::Update(time)) {
            TickResponse::Continue => {
              // main control flow
              if self.config.update_continuously && self.is_focused {
                self.update_frame_counter();
                *control_flow = ControlFlow::Poll;
              } else if let Some(duration) = self.repaint_after.take() {
                *control_flow = ControlFlow::WaitUntil(std::time::Instant::now() + duration);
              } else {
                *control_flow = ControlFlow::Wait;
              }

              self.window.request_redraw();
            }
            TickResponse::Exit => {
              *control_flow = ControlFlow::Exit;
            }
          };

          profiling::finish_frame();
        }
        Event::WindowEvent { window_id, event } if window_id == self.window.id() => {
          body(&mut self, TickEvent::Window(&event));

          match event {
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
          }
        }
        _ => {}
      }
    });

    log::trace!("Stopping engine")
  }

  /// Gets the size of the window
  pub fn window_size(&self) -> (usize, usize) {
    let inner_size = self.window.inner_size();

    (inner_size.width as usize, inner_size.height as usize)
  }

  /// Updates the main window frame counter.
  fn update_frame_counter(&mut self) {
    if self.config.show_fps_in_title {
      let delta_time = self.clock.last_delta_time();

      self.frame_counter.tick(delta_time);

      if self.frame_timer.tick(delta_time) {
        let new_title = format!("{} - FPS: {:.2}", self.config.title, self.frame_counter.fps());

        self.window.set_title(&new_title);
        self.frame_timer.reset();
      }
    }
  }
}

/// Allow the engine to be used in egui rendering.
impl crate::ui::UserInterfaceHost for Engine {
  fn pixels_per_point(&self) -> f32 {
    self.window.scale_factor() as f32
  }

  fn raw_input(&self) -> &egui::RawInput {
    &self.input.raw_input
  }

  // TODO: remove this?
  fn is_key_pressed(&self, key: input::Key) -> bool {
    if let Some(keyboard) = &self.input.keyboard {
      keyboard.is_key_pressed(key)
    } else {
      false
    }
  }

  fn set_exclusive_keyboard_input(&mut self, exclusive: bool) {
    self.input.exclusive_keyboard_input = exclusive;
  }

  fn set_exclusive_pointer_input(&mut self, exclusive: bool) {
    self.input.exclusive_pointer_input = exclusive;
  }

  fn set_cursor_icon(&mut self, cursor_icon: egui::CursorIcon) {
    /// Converts an egui cursor to a winit cursor.
    fn convert_cursor(cursor_icon: egui::CursorIcon) -> Option<glutin::window::CursorIcon> {
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

    // prevent flickering near frame boundary when Windows OS tries to control cursor icon for window resizing
    if self.cursor_icon == cursor_icon {
      return;
    }

    self.cursor_icon = cursor_icon;

    if let Some(cursor_icon) = convert_cursor(cursor_icon) {
      self.window.set_cursor_visible(true);
      self.window.set_cursor_icon(cursor_icon);
    } else {
      self.window.set_cursor_visible(false);
    }
  }

  fn request_redraw(&self) {
    self.window.request_redraw();
  }

  fn request_redraw_after(&mut self, duration: Duration) {
    self.repaint_after = Some(duration);
    self.event_loop_proxy.send_event(()).unwrap();
  }
}
