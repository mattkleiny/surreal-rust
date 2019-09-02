//! A lightweight game engine for Rust.
//!
//! Surreal is designed to be a simple and flexible game engine, not unlike libGDX or MonoGame.
//! 
//! It's opinionated, but small in scope and is intended to form a solid 'library'-like toolkit
//! for constructing small, but highly performant 2d games. A lot of the work is left to the
//! author as to how they'd like to glue things together.

#![allow(dead_code)]
#![allow(unused_variables)]

#![feature(core_intrinsics)]
#![feature(associated_type_defaults)]

extern crate chrono;
extern crate enumflags2;
#[macro_use]
extern crate enumflags2_derive;
#[cfg(feature = "opengl")]
extern crate gl;
extern crate glam;
pub extern crate image;
extern crate imgui;
extern crate imgui_opengl_renderer;
extern crate imgui_sdl2;
extern crate log;
extern crate rand;
pub extern crate rlua;
extern crate sdl2;

pub mod assets;
pub mod culling;
pub mod collections;
pub mod diagnostics;
pub mod entities;
pub mod graphics;
pub mod maths;
pub mod platform;
pub mod scripting;
pub mod utilities;

pub mod prelude {
  //! Import this module to simplify access to Surreal's components.

  pub use crate::assets::*;
  pub use crate::collections::*;
  pub use crate::culling::*;
  pub use crate::diagnostics::*;
  pub use crate::entities::*;
  pub use crate::graphics::*;
  pub use crate::maths::*;
  pub use crate::platform::*;
  pub use crate::scripting::*;
  pub use crate::utilities::*;
}