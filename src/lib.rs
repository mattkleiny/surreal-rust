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
pub mod maths;
pub mod platform;
pub mod scripting;
pub mod utilities;
pub mod window;
pub mod vfs;

/// A handle for a resource created by one of the platform servers.
///
/// Resource IDs can be passed around the application to represent shared
/// platform components without having to have a pointer back to the original provider.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct RID(pub(crate) u32);

/// A generic catch-all error class.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
  Audio,
  Editor,
  General,
  Graphics,
  Input,
  Platform,
}

pub mod prelude {
  pub use crate::assets::*;
  pub use crate::audio::*;
  pub use crate::collections::*;
  pub use crate::diagnostics::*;
  pub use crate::editor::*;
  pub use crate::graphics::*;
  pub use crate::input::*;
  pub use crate::maths::*;
  pub use crate::platform::*;
  pub use crate::scripting::*;
  pub use crate::utilities::*;
  pub use crate::vfs::*;
  pub use crate::window::*;

  pub use super::Error as SurrealError;
}