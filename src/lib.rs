//! A lightweight game engine for Rust.
//!
//! Surreal is designed to be a simple and flexible game engine, not unlike libGDX or MonoGame.
//!
//! It's opinionated, but small in scope and is intended to form a solid 'library'-like toolkit
//! for constructing small but fast 2d games (and maybe 3d someday). A lot of the work is left
//! to the author as to how they'd like to glue things together.

#![allow(incomplete_features)]

#![feature(generic_const_exprs)]
#![feature(const_refs_to_cell)]

// Re-export the macro crate for consumers.
pub extern crate surreal_macros as macros;

pub mod assets;
pub mod audio;
pub mod collections;
pub mod framework;
pub mod graphics;
pub mod input;
pub mod io;
pub mod maths;
pub mod platform;
pub mod utilities;

pub mod prelude {
  //! A prelude for the Surreal engine.
  //!
  //! Import this module to get convenient access to all engine features.

  pub use crate::assets::*;
  pub use crate::audio::*;
  pub use crate::collections::*;
  pub use crate::framework::*;
  pub use crate::graphics::*;
  pub use crate::input::*;
  pub use crate::io::*;
  pub use crate::maths::*;
  pub use crate::platform::*;
  pub use crate::utilities::*;
}