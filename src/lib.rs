//! A lightweight game engine for Rust.
//!
//! Surreal is designed to be a simple and flexible game engine, not unlike
//! libGDX or MonoGame.
//!
//! It's opinionated, but small in scope and is intended to form a solid
//! 'library'-like toolkit for constructing small, but fast 2d games (and maybe
//! 3d someday). A lot of the work is left to the author as to how they'd like
//! to glue things together.

#![allow(dead_code)]
#![allow(unused_variables)]

#[macro_use]
extern crate enumflags2;
#[macro_use]
extern crate smallvec;

pub mod assets;
pub mod audio;
pub mod collections;
pub mod diagnostics;
pub mod editor;
pub mod graphics;
pub mod input;
pub mod io;
pub mod maths;
pub mod platform;
pub mod scripting;
pub mod ui;
pub mod utilities;

/// Represents a general error with the engine.
#[derive(Debug)]
pub enum Error {
  General,
  Asset(assets::Error),
  Audio(audio::Error),
  Graphics(graphics::Error),
  Input(input::Error),
  IO(io::Error),
  Platform(platform::Error),
}

pub mod prelude {
  //! Import this module to enable simple access to the engine.

  pub use crate::assets::*;
  pub use crate::audio::*;
  pub use crate::collections::*;
  pub use crate::diagnostics::*;
  pub use crate::editor::*;
  pub use crate::graphics::*;
  pub use crate::input::*;
  pub use crate::io::*;
  pub use crate::maths::*;
  pub use crate::platform::*;
  pub use crate::scripting::*;
  pub use crate::ui::*;
  pub use crate::utilities::*;

  pub use super::Error as SurrealError;
}
