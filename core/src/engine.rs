//! Game framework for Surreal.
//!
//! Bootstrapping and other framework systems for common projects.

use std::time::Duration;

use winit::{
  dpi::LogicalSize,
  event::WindowEvent,
  event_loop::{ControlFlow, EventLoop},
  window::{Window, WindowBuilder},
};

use crate::{
  assets::AssetManager,
  audio::AudioServer,
  diagnostics::{self, ConsoleLogger, LevelFilter},
  graphics::{GraphicsServer, Image, ImageFormat, Renderer},
  input::InputServer,
  maths::{uvec2, vec2},
  physics::PhysicsServer,
  scene::SceneGraph,
  utilities::{DeltaClock, FrameCounter, IntervalTimer, TimeSpan},
};

/// Configuration for the [`Engine`].
///
/// This struct defines how to set-up the game and initial settings.
#[derive(Clone, Debug)]
pub struct EngineConfig {
  pub title: String,
  pub size: (u32, u32),
  pub vsync_enabled: bool,
  pub samples: u8,
  pub update_continuously: bool,
  pub is_window_visible: bool,
  pub transparent_window: bool,
  pub show_fps_in_title: bool,
  pub log_level: LevelFilter,
  pub icon: Option<&'static [u8]>,
}

impl Default for EngineConfig {
  fn default() -> Self {
    Self {
      title: "Surreal".to_string(),
      size: (1280, 720),
      vsync_enabled: true,
      samples: 1,
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
  /// An event from the underlying window.
  Window(&'a WindowEvent<'a>),
}

/// Represents an application that can be used in an [`Engine`].
#[allow(unused_variables)]
pub trait Application: Sized {
  /// Builds the [`Application`] instance.
  fn new(engine: &mut Engine, assets: &mut AssetManager) -> crate::Result<Self>;

  /// Called when the application is to be updated.
  fn on_update(&mut self, engine: &mut Engine, time: GameTime) {}

  /// Called when the application is to be drawn.
  fn on_draw(&mut self, engine: &mut Engine, time: GameTime) {}

  /// Invoked when a [`WindowEvent`] is received.
  fn on_window_event(&mut self, engine: &mut Engine, event: &WindowEvent) {}

  /// Notifies the application of an [`TickEvent`]. l
  fn notify(&mut self, engine: &mut Engine, event: TickEvent) {
    match event {
      TickEvent::Update(time) => self.on_update(engine, time),
      TickEvent::Draw(time) => self.on_draw(engine, time),
      TickEvent::Window(event) => self.on_window_event(engine, event),
    }
  }
}

/// A builder pattern over the [`Engine`] and [`EngineConfig`].
#[derive(Default)]
pub struct EngineBuilder {
  config: EngineConfig,
}

impl EngineBuilder {
  pub fn with_title(mut self, title: &str) -> Self {
    self.config.title = title.to_string();
    self
  }

  pub fn with_size(mut self, size: (u32, u32)) -> Self {
    self.config.size = size;
    self
  }

  pub fn with_vsync(mut self, vsync: bool) -> Self {
    self.config.vsync_enabled = vsync;
    self
  }

  pub fn with_samples(mut self, samples: u8) -> Self {
    self.config.samples = samples;
    self
  }

  pub fn with_update_continuously(mut self, update_continuously: bool) -> Self {
    self.config.update_continuously = update_continuously;
    self
  }

  pub fn with_window_visible(mut self, is_window_visible: bool) -> Self {
    self.config.is_window_visible = is_window_visible;
    self
  }

  pub fn with_transparent_window(mut self, transparent_window: bool) -> Self {
    self.config.transparent_window = transparent_window;
    self
  }

  pub fn with_fps_in_title(mut self, fps_in_title: bool) -> Self {
    self.config.show_fps_in_title = fps_in_title;
    self
  }

  pub fn with_log_level(mut self, log_level: LevelFilter) -> Self {
    self.config.log_level = log_level;
    self
  }

  pub fn with_icon(mut self, icon: &'static [u8]) -> Self {
    self.config.icon = Some(icon);
    self
  }

  /// Starts an [`Application`] on the resultant [`Engine`].
  pub fn start_application<A: Application>(self) -> crate::Result<()> {
    self.start(|mut engine, mut assets| {
      let mut application = A::new(&mut engine, &mut assets).expect("Failed to create application");

      engine.run(|engine, event| {
        application.notify(engine, event);
      })
    })
  }

  /// Starts a [`SceneGraph`] on the resultant [`Engine`].
  pub fn start_scene(
    self,
    setup: impl Fn(&Engine, &AssetManager) -> crate::Result<SceneGraph>,
  ) -> crate::Result<()> {
    self.start(|engine, assets| {
      let mut scene = setup(&engine, &assets)?;
      let mut renderer = Renderer::new(&engine.graphics);

      engine.run_variable_step(|_, time| {
        renderer.begin_frame();

        scene.update(time.delta_time);
        scene.draw(&mut renderer);

        renderer.end_frame();
      })
    })
  }

  /// Starts the [`Engine`] with the given callback.
  pub fn start(
    self,
    setup: impl FnOnce(Engine, AssetManager) -> crate::Result<()>,
  ) -> crate::Result<()> {
    Engine::start(self.config, setup)
  }

  /// Builds the resultant [`Engine`].
  pub fn build(self) -> crate::Result<Engine> {
    Engine::new(self.config)
  }
}

/// The core engine for Surreal.
///
/// This struct manages core systems and facilitates the main game loop.
pub struct Engine {
  // core systems
  pub audio: AudioServer,
  pub graphics: GraphicsServer,
  pub physics: PhysicsServer,
  pub input: InputServer,

  // window management
  config: EngineConfig,
  pub window: Window,
  cursor_icon: egui::CursorIcon,
  event_loop: Option<EventLoop<()>>,
  is_focused: bool,

  // timing
  clock: DeltaClock,
  frame_timer: IntervalTimer,
  frame_counter: FrameCounter,
  is_quitting: bool,
}

impl Engine {
  /// Creates a new engine, bootstrapping all core systems and display.
  pub fn new(config: EngineConfig) -> crate::Result<Self> {
    // prepare the main window and event loop
    let event_loop = EventLoop::new();

    log::trace!("Building main window");

    let window = WindowBuilder::new()
      .with_title(&config.title)
      .with_inner_size(LogicalSize::new(config.size.0, config.size.1))
      .with_resizable(true)
      .with_transparent(config.transparent_window)
      .with_visible(config.is_window_visible)
      .with_window_icon(config.icon.map(|buffer| {
        Image::from_buffer(buffer, ImageFormat::Ico)
          .and_then(|image| image.to_icon())
          .expect("Failed to convert icon from raw image")
      }))
      .build(&event_loop)?;

    let audio = AudioServer::rodio();
    let graphics = GraphicsServer::opengl(&window, config.vsync_enabled, config.samples)?;
    let physics = PhysicsServer::default();
    let input = InputServer::new(window.scale_factor() as f32);

    Ok(Self {
      // servers
      audio,
      graphics,
      physics,
      input,

      // window management
      config,
      window,
      cursor_icon: egui::CursorIcon::None,
      event_loop: Some(event_loop),
      is_focused: true,

      // timing
      clock: DeltaClock::new(),
      frame_timer: IntervalTimer::new(TimeSpan::from_seconds(1.)),
      frame_counter: FrameCounter::new(32),
      is_quitting: false,
    })
  }

  /// Starts the engine with the given configuration.
  pub fn start(
    configuration: EngineConfig,
    setup: impl FnOnce(Engine, AssetManager) -> crate::Result<()>,
  ) -> crate::Result<()> {
    use crate::graphics::*;

    // set-up diagnostics
    ConsoleLogger::install(configuration.log_level);

    // set-up core engine
    log::trace!("Starting engine");

    let engine = Engine::new(configuration)?;
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

    setup(engine, assets)
  }

  /// Runs a variable step game loop against the engine.
  ///
  /// This method will block until the game is closed.
  pub fn run_variable_step(self, mut body: impl FnMut(&mut Engine, GameTime)) -> crate::Result<()> {
    self.run(move |engine, event| {
      if let TickEvent::Update(time) = event {
        body(engine, time)
      }
    })
  }

  /// Runs the given delegate as the main loop for the engine.
  ///
  /// This method will block until the game is closed.
  pub fn run(mut self, mut body: impl FnMut(&mut Self, TickEvent)) -> crate::Result<()> {
    use winit::{event::*, platform::run_return::EventLoopExtRunReturn};

    log::trace!("Entering main event loop");

    // use this hack to unpack the event loop out of 'self' and then remove the
    // 'static lifetime bound on run_return so that body can access things in
    // self without lifetime woes.
    let mut event_loop = self
      .event_loop
      .take()
      .expect("The engine has already run once and cannot be started again");

    event_loop.run_return(move |event, _, control_flow| {
      match event {
        Event::MainEventsCleared => {
          // update core systems
          diagnostics::profile_scope!("Update");

          let time = GameTime {
            delta_time: self.clock.tick(),
            total_time: self.clock.total_time(),
          };

          body(&mut self, TickEvent::Update(time));

          // main control flow
          if self.config.update_continuously && self.is_focused {
            self.update_frame_counter();
            *control_flow = ControlFlow::Poll;
          } else {
            *control_flow = ControlFlow::Wait;
          }

          self.window.request_redraw();
          diagnostics::finish_frame();

          if self.is_quitting {
            *control_flow = ControlFlow::Exit;
          }
        }
        Event::RedrawRequested(window_id) => {
          if window_id == self.window.id() {
            diagnostics::profile_scope!("Redraw");

            // update graphics and run draw loop
            self.graphics.begin_frame();

            let time = GameTime {
              delta_time: self.clock.last_delta_time(),
              total_time: self.clock.total_time(),
            };

            body(&mut self, TickEvent::Draw(time));
            self.input.tick(); // TODO: why does this need to live here?

            self.graphics.end_frame();
          }
        }
        Event::WindowEvent { window_id, event } if window_id == self.window.id() => {
          // pass raw events down the application
          body(&mut self, TickEvent::Window(&event));

          // also apply some of our own processing
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
              self.graphics.set_viewport_size(size);

              log::trace!("Window resized to {}x{}", size.width, size.height);
            }
            WindowEvent::CloseRequested => {
              log::trace!("Window close requested");

              *control_flow = ControlFlow::Exit;
            }
            _ => {}
          }
        }
        _ => {}
      }
    });

    log::trace!("Stopping engine");

    Ok(())
  }

  /// Gets the title of the window.
  pub fn title(&self) -> &str {
    &self.config.title
  }

  /// Sets the title of the window.
  pub fn set_title(&mut self, title: &str) {
    self.config.title = title.to_string();
    self.window.set_title(title);
  }

  /// Determines if the cursor is currently visible.
  pub fn set_cursor_visible(&mut self, visible: bool) {
    self.window.set_cursor_visible(visible);
  }

  /// Quits the engine at the next loop.
  pub fn quit(&mut self) {
    self.is_quitting = true;
  }

  /// Updates the main window frame counter.
  fn update_frame_counter(&mut self) {
    if self.config.show_fps_in_title {
      let delta_time = self.clock.last_delta_time();

      self.frame_counter.tick(delta_time);

      if self.frame_timer.tick(delta_time) {
        let new_title = format!(
          "{} - FPS: {:.2}",
          self.config.title,
          self.frame_counter.fps()
        );

        self.window.set_title(&new_title);
        self.frame_timer.reset();
      }
    }
  }
}

/// Allow the engine to be used hooked into `egui` rendering.
impl crate::ui::UserInterfaceHost for Engine {
  fn pixels_per_point(&self) -> f32 {
    self.window.scale_factor() as f32
  }

  fn raw_input(&self) -> &egui::RawInput {
    &self.input.raw_input
  }

  fn set_exclusive_keyboard_input(&mut self, exclusive: bool) {
    self.input.exclusive_keyboard_input = exclusive;
  }

  fn set_exclusive_pointer_input(&mut self, exclusive: bool) {
    self.input.exclusive_pointer_input = exclusive;
  }

  fn set_cursor_icon(&mut self, cursor_icon: egui::CursorIcon) {
    /// Converts an egui cursor to a winit cursor.
    fn convert_cursor(cursor_icon: egui::CursorIcon) -> Option<winit::window::CursorIcon> {
      match cursor_icon {
        egui::CursorIcon::None => None,

        egui::CursorIcon::Alias => Some(winit::window::CursorIcon::Alias),
        egui::CursorIcon::AllScroll => Some(winit::window::CursorIcon::AllScroll),
        egui::CursorIcon::Cell => Some(winit::window::CursorIcon::Cell),
        egui::CursorIcon::ContextMenu => Some(winit::window::CursorIcon::ContextMenu),
        egui::CursorIcon::Copy => Some(winit::window::CursorIcon::Copy),
        egui::CursorIcon::Crosshair => Some(winit::window::CursorIcon::Crosshair),
        egui::CursorIcon::Default => Some(winit::window::CursorIcon::Default),
        egui::CursorIcon::Grab => Some(winit::window::CursorIcon::Grab),
        egui::CursorIcon::Grabbing => Some(winit::window::CursorIcon::Grabbing),
        egui::CursorIcon::Help => Some(winit::window::CursorIcon::Help),
        egui::CursorIcon::Move => Some(winit::window::CursorIcon::Move),
        egui::CursorIcon::NoDrop => Some(winit::window::CursorIcon::NoDrop),
        egui::CursorIcon::NotAllowed => Some(winit::window::CursorIcon::NotAllowed),
        egui::CursorIcon::PointingHand => Some(winit::window::CursorIcon::Hand),
        egui::CursorIcon::Progress => Some(winit::window::CursorIcon::Progress),

        egui::CursorIcon::ResizeHorizontal => Some(winit::window::CursorIcon::EwResize),
        egui::CursorIcon::ResizeNeSw => Some(winit::window::CursorIcon::NeswResize),
        egui::CursorIcon::ResizeNwSe => Some(winit::window::CursorIcon::NwseResize),
        egui::CursorIcon::ResizeVertical => Some(winit::window::CursorIcon::NsResize),

        egui::CursorIcon::ResizeEast => Some(winit::window::CursorIcon::EResize),
        egui::CursorIcon::ResizeSouthEast => Some(winit::window::CursorIcon::SeResize),
        egui::CursorIcon::ResizeSouth => Some(winit::window::CursorIcon::SResize),
        egui::CursorIcon::ResizeSouthWest => Some(winit::window::CursorIcon::SwResize),
        egui::CursorIcon::ResizeWest => Some(winit::window::CursorIcon::WResize),
        egui::CursorIcon::ResizeNorthWest => Some(winit::window::CursorIcon::NwResize),
        egui::CursorIcon::ResizeNorth => Some(winit::window::CursorIcon::NResize),
        egui::CursorIcon::ResizeNorthEast => Some(winit::window::CursorIcon::NeResize),
        egui::CursorIcon::ResizeColumn => Some(winit::window::CursorIcon::ColResize),
        egui::CursorIcon::ResizeRow => Some(winit::window::CursorIcon::RowResize),

        egui::CursorIcon::Text => Some(winit::window::CursorIcon::Text),
        egui::CursorIcon::VerticalText => Some(winit::window::CursorIcon::VerticalText),
        egui::CursorIcon::Wait => Some(winit::window::CursorIcon::Wait),
        egui::CursorIcon::ZoomIn => Some(winit::window::CursorIcon::ZoomIn),
        egui::CursorIcon::ZoomOut => Some(winit::window::CursorIcon::ZoomOut),
      }
    }

    // prevent flickering near frame boundary when Windows OS tries to control
    // cursor icon for window resizing
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

  fn request_redraw(&mut self) {
    self.window.request_redraw();
  }

  fn request_redraw_after(&mut self, _duration: Duration) {
    self.window.request_redraw(); // TODO: implement me
  }
}
