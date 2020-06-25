//! Platform abstractions and utilities.
//!
//! We export a minimum of our platform layer (Luminance) in order to make it simpler to use.

use std::time::Instant;

use luminance::context::*;
use luminance::pipeline::*;
use luminance::pixel::*;
use luminance::texture::*;
use luminance_glfw::*;

use crate::utilities::Clock;

// TODO: think about how to implement hot-reloading and other niceties

/// Configuration for the core game loop and the platform initialization.
pub struct Config<S> {
  pub title: &'static str,
  pub size: (u32, u32),
  pub clear_color: [f32; 4],
  pub max_delta: f32,
  pub state: S,
}

/// A renderer abstraction for our platform.
///
/// This hides the inner implementation details of Luminance and makes
/// it simpler to get sprites on the screen.
pub struct Frame<'a> {
  shading: ShadingGate<'a, GlfwSurface>
}

/// Contains information on the game's timing state.
#[derive(Copy, Clone, Debug)]
pub struct GameTime {
  pub delta_time: f32,
  pub frame: usize,
}

/// Represents an error in the platform layer.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
  GraphicsError,
  TextureError,
}

impl From<GlfwSurfaceError> for Error {
  fn from(error: GlfwSurfaceError) -> Self {
    match error {
      GlfwSurfaceError::InitError(_) => Error::GraphicsError,
      GlfwSurfaceError::WindowCreationFailed => Error::GraphicsError,
      GlfwSurfaceError::NoPrimaryMonitor => Error::GraphicsError,
      GlfwSurfaceError::NoVideoMode => Error::GraphicsError,
      GlfwSurfaceError::GraphicsStateError(_) => Error::GraphicsError,
    }
  }
}

impl From<TextureError> for Error {
  fn from(error: TextureError) -> Self {
    match error {
      TextureError::TextureStorageCreationFailed(_) => Error::TextureError,
      TextureError::NotEnoughPixels(_, _) => Error::TextureError,
      TextureError::UnsupportedPixelFormat(_) => Error::TextureError,
    }
  }
}

/// Runs the game with the given configuration.
pub fn run<S, I, U, D>(
  mut config: Config<S>,
  mut input: I,
  mut update: U,
  mut draw: D,
) -> Result<(), Error>
  where I: FnMut(&mut S, GameTime) -> (),
        U: FnMut(&mut S, GameTime) -> (),
        D: FnMut(&mut S, GameTime, Frame) -> () {
  // build our window, this thing also handles our window events
  let mut surface = GlfwSurface::new(
    WindowDim::Windowed(config.size.0, config.size.1),
    config.title,
    WindowOpt::default(),
  )?;

  let state = &mut config.state;
  let back_buffer = surface.back_buffer()?;
  let mut clock = Clock::new(config.max_delta);
  let mut frame = 0;

  let texture = Texture::<Dim2, RGB8UI>::new(&mut surface, [256, 144], 0, Sampler {
    wrap_r: Wrap::ClampToEdge,
    wrap_s: Wrap::ClampToEdge,
    wrap_t: Wrap::ClampToEdge,
    min_filter: MinFilter::Nearest,
    mag_filter: MagFilter::Nearest,
    depth_comparison: None,
  })?;

  texture.clear(GenMipmaps::No, (255, 255, 255))?;

  'game: loop {
    // update the clock
    let now = Instant::now().elapsed().as_secs();
    let time = GameTime { delta_time: clock.tick(now, 60u64), frame };

    frame += 1;

    // update the underlying window
    for event in surface.poll_events() {
      match event {
        WindowEvent::Close => break 'game,
        WindowEvent::Key(_, _, _, _) => input(state, time),
        _ => (),
      }
    }

    // update the game simulation
    update(state, time);

    // draw this frame
    surface.pipeline_builder().pipeline(
      &back_buffer,
      &PipelineState::default().set_clear_color(config.clear_color),
      |_, shading| {
        draw(state, time, Frame { shading });
      },
    );

    surface.swap_buffers();
  }

  Ok(())
}

