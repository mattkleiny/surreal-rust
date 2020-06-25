//! Platform abstractions and utilities.
//!
//! We export a minimum of our platform layer.

use std::time::Instant;

use sdl2::event::Event;
use sdl2::render::WindowCanvas;

use crate::graphics::{Color, GraphicsDevice};
use crate::utilities::Clock;

// TODO: think about how to implement hot-reloading and other niceties

/// Configuration for the core game loop and the platform initialization.
pub struct Config<S> {
  pub title: &'static str,
  pub size: (u32, u32),
  pub max_delta: f32,
  pub state: S,
}

/// A renderer abstraction for our platform.
///
/// This hides the inner implementation details of Luminance and makes
/// it simpler to get sprites on the screen.
pub struct Frame<'a> {
  canvas: &'a mut WindowCanvas,
}

impl<'a> GraphicsDevice for Frame<'a> {
  fn clear(&mut self, color: Color) {
    let color: (u8, u8, u8, u8) = color.into();
    self.canvas.set_draw_color(color);
    self.canvas.clear();
  }
}

/// Contains information on the game's timing state.
#[derive(Copy, Clone, Debug)]
pub struct GameTime {
  pub delta_time: f32,
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
  let context = sdl2::init()?;
  let video = context.video()?;

  let window = video.window(config.title, config.size.0, config.size.1)
      .position_centered()
      .resizable()
      .build()?;

  let mut canvas = window.into_canvas()
      .present_vsync()
      .accelerated()
      .build()?;

  let mut pump = context.event_pump()?;
  let mut clock = Clock::new(config.max_delta);

  'running: loop {
    for event in pump.poll_iter() {
      match event {
        Event::Quit { .. } => break 'running,
        _ => {}
      }
    }

    let time = GameTime {
      delta_time: clock.tick(Instant::now().elapsed().as_secs(), 60),
    };

    let frame = Frame { canvas: &mut canvas };

    input(&mut config.state, time);
    update(&mut config.state, time);
    draw(&mut config.state, time, frame);

    canvas.present();
  }

  Ok(())
}

/// Represents an error in the platform layer.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
  GenericFailure
}

impl From<String> for Error {
  fn from(_: String) -> Self {
    Error::GenericFailure
  }
}

impl From<sdl2::Error> for Error {
  fn from(_: sdl2::Error) -> Self {
    Error::GenericFailure
  }
}

impl From<sdl2::video::WindowBuildError> for Error {
  fn from(_: sdl2::video::WindowBuildError) -> Self {
    Error::GenericFailure
  }
}

impl From<sdl2::IntegerOrSdlError> for Error {
  fn from(_: sdl2::IntegerOrSdlError) -> Self {
    Error::GenericFailure
  }
}
