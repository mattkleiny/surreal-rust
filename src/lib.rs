//! A lightweight game engine for Rust.
//!
//! Surreal is designed to be a simple and flexible game engine, not unlike libGDX or MonoGame.
//!
//! It's opinionated, but small in scope and is intended to form a solid 'library'-like toolkit
//! for constructing small but fast 2d games (and maybe 3d someday). A lot of the work is left
//! to the author as to how they'd like to glue things together.

#![allow(dead_code)]
#![allow(unused_variables)]

extern crate core;

pub mod assets;
pub mod audio;
pub mod collections;
pub mod ecs;
pub mod graphics;
pub mod input;
pub mod io;
pub mod maths;
pub mod platform;
pub mod utilities;

pub mod prelude {
  pub use crate::assets::*;
  pub use crate::audio::*;
  pub use crate::collections::*;
  pub use crate::ecs::*;
  pub use crate::graphics::*;
  pub use crate::input::*;
  pub use crate::io::*;
  pub use crate::maths::*;
  pub use crate::platform::*;
  pub use crate::utilities::*;

  pub use super::Game;
}

/// A utility context for bootstrapping games.
pub struct Game<P> {
  /// The underlying backend platform for the game.
  pub platform: P,

  /// Is the game still running? false if we should end at the end of the frame.
  is_running: bool,
}

impl<P> Game<P> where P: platform::Platform {
  /// Starts a new game with the given platform.
  pub fn start(platform: P, mut setup: impl FnMut(Game<P>)) {
    let game = Game {
      platform,
      is_running: false,
    };

    setup(game);
  }

  /// Runs the game loop in a variable step fashion.
  pub fn run_variable_step(mut self, mut tick: impl FnMut(&mut Game<P>, utilities::GameTime)) {
    use crate::utilities::{Clock, GameTime};

    let mut timer = Clock::new();

    while self.is_running {
      let time = GameTime {
        delta_time: timer.tick(),
        total_time: timer.total_time(),
      };

      self.platform.tick();
      tick(&mut self, time);
    };
  }

  /// Runs the game loop in a fixed step fashion.
  pub fn run_fixed_step(mut self, mut update: impl FnMut(&mut Game<P>, utilities::GameTime)) {
    todo!()
  }

  /// Exits the game at the end of the frame.
  pub fn exit(&mut self) {
    self.is_running = false;
  }
}
