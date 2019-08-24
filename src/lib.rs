//! A lightweight game engine for Rust.

#![allow(dead_code)]

extern crate glam;
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
  pub use crate::audio::*;
  pub use crate::collections::*;
  pub use crate::diagnostics::*;
  pub use crate::framework::*;
  pub use crate::graphics::*;
  pub use crate::input::*;
  pub use crate::maths::*;
  pub use crate::platform::*;
  pub use crate::scripting::*;
  pub use crate::timing::*;

  #[cfg(feature = "editor")]
  pub use crate::editor::*;
}

/// A common result type for the entire module.
pub type Result<T> = std::result::Result<T, Error>;

/// A common error type for the entire module.
pub type Error = String;