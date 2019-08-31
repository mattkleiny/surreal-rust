//! A lightweight game engine for Rust.

#![allow(dead_code)]
#![allow(unused_variables)]

extern crate chrono;
extern crate enumflags2;
#[macro_use]
extern crate enumflags2_derive;
#[cfg(feature = "opengl")]
extern crate gl;
extern crate glam;
extern crate hotwatch;
extern crate imgui;
extern crate imgui_opengl_renderer;
extern crate imgui_sdl2;
extern crate log;
extern crate rand;
extern crate rlua;
extern crate sdl2;

pub mod diagnostics;
pub mod framework;
pub mod collections;
pub mod graphics;
pub mod maths;
pub mod platform;
pub mod scripting;
pub mod timing;

pub mod prelude {
  pub use crate::diagnostics::*;
  pub use crate::framework::*;
  pub use crate::collections::*;
  pub use crate::graphics::*;
  pub use crate::maths::*;
  pub use crate::platform::*;
  pub use crate::scripting::*;
  pub use crate::timing::*;
}