//! Platform abstractions and utilities.

use std::process::exit;

use luminance::context::GraphicsContext;
use luminance::pipeline::PipelineState;
use luminance_glfw::{Action, GlfwSurface, Key, Surface, WindowDim, WindowEvent, WindowOpt};

/// Configuration for the platform.
#[derive(Clone, Debug)]
pub struct Config {
  pub title: &'static str,
  pub size: (u32, u32),
}

/// Executes the engine with the given configuration.
pub fn run(config: Config) {
  let surface = GlfwSurface::new(
    WindowDim::Windowed(config.size.0, config.size.1),
    config.title,
    WindowOpt::default(),
  );

  match surface {
    Ok(mut surface) => {
      let back_buffer = surface.back_buffer().unwrap();

      'app: loop {
        for event in surface.poll_events() {
          match event {
            WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => break 'app,
            _ => (),
          }
        }

        surface.pipeline_builder().pipeline(
          &back_buffer,
          &PipelineState::default().set_clear_color([0.7, 0.1, 0.5, 1.]),
          |_, _| (),
        );

        surface.swap_buffers();
      }
    }
    Err(e) => {
      eprintln!("Cannot create graphics surface:\n{}", e);
      exit(-1);
    }
  }
}
