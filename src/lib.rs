//! A lightweight game engine for Rust.

#![allow(dead_code)]

extern crate chrono;
extern crate enumflags2;
#[macro_use]
extern crate enumflags2_derive;
extern crate gl;
extern crate glam;
extern crate imgui;
extern crate imgui_opengl_renderer;
extern crate imgui_sdl2;
extern crate hotwatch;
extern crate log;
extern crate rand;
extern crate rlua;
extern crate sdl2;
extern crate specs;

pub mod audio;
pub mod diagnostics;
pub mod collections;
pub mod framework;
pub mod graphics;
pub mod input;
pub mod maths;
pub mod platform;
pub mod scripting;
pub mod timing;

#[cfg(feature = "editor")]
pub mod editor;

/// Re-export most public modules for easier consumption.
pub mod prelude {
  pub use crate::collections::*;
  pub use crate::diagnostics::*;
  #[cfg(feature = "editor")]
  pub use crate::editor::*;
  pub use crate::framework::*;
  pub use crate::graphics::*;
  pub use crate::input::*;
  pub use crate::maths::*;
  pub use crate::platform::*;
  pub use crate::scripting::*;
  pub use crate::timing::*;
}

/// A common result type for the entire module.
pub type Result<T> = std::result::Result<T, Error>;

/// A common error type for the entire module.
pub type Error = String;