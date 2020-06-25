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
pub mod editor;
pub mod graphics;
pub mod maths;
pub mod platform;
pub mod scripting;
pub mod servers;
pub mod utilities;

pub mod prelude {
  pub use crate::collections::*;
  pub use crate::diagnostics::*;
  pub use crate::editor::*;
  pub use crate::graphics::*;
  pub use crate::maths::*;
  pub use crate::platform::*;
  pub use crate::scripting::*;
  pub use crate::utilities::*;
}