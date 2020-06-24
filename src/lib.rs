//! A lightweight game engine for Rust.
//!
//! Surreal is designed to be a simple and flexible game engine, not unlike libGDX or MonoGame.
//! 
//! It's opinionated, but small in scope and is intended to form a solid 'library'-like toolkit
//! for constructing small, but highly performant 2d games. A lot of the work is left to the
//! author as to how they'd like to glue things together.

#![allow(dead_code)]
#![allow(unused_variables)]

#[macro_use]
extern crate enumflags2;
#[macro_use]
extern crate smallvec;

pub mod collections;
pub mod diagnostics;
#[cfg(feature = "editor")]
pub mod editor;
#[cfg(feature = "graphics")]
pub mod graphics;
pub mod maths;
pub mod platform;
#[cfg(feature = "scripting")]
pub mod scripting;
pub mod utilities;
#[cfg(feature = "ui")]
pub mod ui;

pub mod prelude {
  pub use crate::collections::*;
  pub use crate::diagnostics::*;
  #[cfg(feature = "editor")]
  pub use crate::editor::*;
  #[cfg(feature = "graphics")]
  pub use crate::graphics::*;
  pub use crate::maths::*;
  pub use crate::platform::*;
  #[cfg(feature = "scripting")]
  pub use crate::scripting::*;
  pub use crate::utilities::*;
  #[cfg(feature = "ui")]
  pub use crate::ui::*;
}