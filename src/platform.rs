//! Platform abstractions and utilities.

use luminance::context::GraphicsContext;
use luminance::pipeline::PipelineState;
use luminance_glfw::{Action, GlfwSurface, Key, Surface, WindowDim, WindowEvent, WindowOpt};

/// Configuration for the platform.
#[derive(Clone)]
pub struct Config<S> {
  pub title: &'static str,
  pub size: (u32, u32),
  pub callback: fn(f32, &mut S),
  pub state: S,
}

/// Executes the engine with the given configuration.
pub fn run<S>(mut config: Config<S>) {
  let mut surface = GlfwSurface::new(
    WindowDim::Windowed(config.size.0, config.size.1),
    config.title,
    WindowOpt::default(),
  ).unwrap_or_else(|error| panic!("Failed to create surface: {:?}", error));

  let back_buffer = surface.back_buffer().unwrap();

  'app: loop {
    for event in surface.poll_events() {
      match event {
        WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => break 'app,
        _ => (),
      }
    }

    (config.callback)(0.16, &mut config.state);

    surface.pipeline_builder().pipeline(
      &back_buffer,
      &PipelineState::default().set_clear_color([0.7, 0.1, 0.5, 1.]),
      |_, _| (),
    );

    surface.swap_buffers();
  }
}
