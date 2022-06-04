//! Game framework for Surreal.
//!
//! Bootstrapping and other framework systems for common projects.

pub use ecs::*;

mod ecs;

use glutin::{window::Window, ContextBuilder};
use log::LevelFilter;
use winit::{
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
  pub assets: AssetManager,
  pub audio: AudioServer,
  pub graphics: GraphicsServer,
  pub input: InputBackend,

  // window management
  config: Configuration,
  window: Window,
  event_loop: Option<EventLoop<()>>,
  is_focused: bool,

  // timing
  clock: Clock,
  frame_timer: IntervalTimer,
  frame_counter: FrameCounter,
}

impl Engine {
  /// Starts the engine with the given configuration.
  pub fn start(configuration: Configuration, mut setup: impl FnMut(Engine)) {
    use crate::graphics::*;

    // set-up diagnostics
    ConsoleLogger::install(configuration.log_level);
    profiling::register_thread!("Main Thread");

    // set-up core engine
    log::trace!("Starting engine");

    let mut engine = Engine::new(configuration);

    let graphics = &engine.graphics;
    let assets = &mut engine.assets;

    // set-up asset manager
    log::trace!("Registering asset loaders");

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

    setup(engine);
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
      assets: AssetManager::new(),
      audio: AudioServer::new(Box::new(audio)),
      graphics: GraphicsServer::new(Box::new(graphics)),
      input: InputBackend::new(pixels_per_point),

      // window management
      config,
      window,
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
    use winit::event::*;
    use winit::platform::run_return::EventLoopExtRunReturn;

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
