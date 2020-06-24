//! Platform abstractions and utilities.
//!
//! We export a minimum of our platform layer (Luminance) in order to make it simpler to use.

use std::time::Instant;

use luminance::context::*;
use luminance::pipeline::*;
use luminance_glfw::*;

use crate::utilities::Clock;

/// Represents an error in the platform player.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
  FailedToCreate,
  Unknown,
}

impl From<GlfwSurfaceError> for Error {
  fn from(error: GlfwSurfaceError) -> Self {
    match error {
      GlfwSurfaceError::InitError(_) => Error::FailedToCreate,
      GlfwSurfaceError::WindowCreationFailed => Error::FailedToCreate,
      GlfwSurfaceError::NoPrimaryMonitor => Error::FailedToCreate,
      GlfwSurfaceError::NoVideoMode => Error::FailedToCreate,
      GlfwSurfaceError::GraphicsStateError(_) => Error::FailedToCreate,
    }
  }
}

/// A renderer abstraction for our platform.
///
/// This hides the inner implementation details of Luminance and makes
/// it simpler to get sprites on the screen.
pub struct Frame<'a> {
  shading: ShadingGate<'a, GlfwSurface>
}

/// Configuration for the core game loop.
pub struct Config<S> {
  pub title: &'static str,
  pub size: (u32, u32),
  pub clear_color: [f32; 4],
  pub max_delta: f32,
  pub state: S,
}

/// Runs the game with the given configuration.
pub fn run<S, T, D>(mut config: Config<S>, mut tick: T, mut draw: D) -> Result<(), Error>
  where T: FnMut(&mut S, f32) -> (),
        D: FnMut(&mut S, f32, Frame) -> () {
  // build our window, this thing also handles our window events
  let mut surface = GlfwSurface::new(
    WindowDim::Windowed(config.size.0, config.size.1),
    config.title,
    WindowOpt::default(),
  )?;

  let mut clock = Clock::new(config.max_delta);

  let back_buffer = surface.back_buffer()?;

  // core game loop
  'app: loop {
    for event in surface.poll_events() {
      match event {
        WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => break 'app,
        _ => (),
      }
    }

    // update the clock
    let now = Instant::now().elapsed().as_secs();
    let delta = clock.tick(now, 60u64);

    // update this frame
    tick(&mut config.state, delta);

    // render this frame
    surface.pipeline_builder().pipeline(
      &back_buffer,
      &PipelineState::default().set_clear_color(config.clear_color),
      |_, mut shading| {
        draw(&mut config.state, delta, Frame { shading });
      },
    );

    surface.swap_buffers();
  }

  Ok(())
}

