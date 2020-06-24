//! Platform abstractions and utilities.
//!
//! We export a minimum of our platform layer (Luminance) in order to make it simpler to use.

use luminance::context::*;
pub use luminance::pipeline::*;
use luminance_glfw::*;
pub use luminance_glfw::{GlfwSurface, GlfwSurfaceError};
use crate::utilities::Clock;

/// Configuration for the core game loop.
pub struct Config<S> {
  pub title: &'static str,
  pub size: (u32, u32),
  pub clear_color: [f32; 4],
  pub state: S,
}

/// Runs the game with the given configuration.
pub fn run<S, T, D>(mut config: Config<S>, mut tick: T, mut draw: D) -> Result<(), GlfwSurfaceError>
  where T: FnMut(&mut S, f32) -> (),
        D: FnMut(&mut S, f32, &mut ShadingGate<GlfwSurface>) -> () {
  // build our window, this thing also handles our window events
  let mut surface = GlfwSurface::new(
    WindowDim::Windowed(config.size.0, config.size.1),
    config.title,
    WindowOpt::default(),
  )?;

  let mut clock = Clock::new(0.32);
  let mut state = &mut config.state;
  let back_buffer = surface.back_buffer()?;

  // core game loop
  'app: loop {
    for event in surface.poll_events() {
      match event {
        WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => break 'app,
        _ => (),
      }
    }

    // update this frame
    let delta = 0.16; // TODO: use the clock for this instead
    tick(state, delta);

    // render this frame
    surface.pipeline_builder().pipeline(
      &back_buffer,
      &PipelineState::default().set_clear_color(config.clear_color),
      |_, mut shading| {
        draw(state, delta, &mut shading);
      },
    );

    surface.swap_buffers();
  }

  Ok(())
}

