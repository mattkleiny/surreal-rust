//! A lightweight game engine for Rust.
//!
//! Surreal is designed to be a simple and flexible game engine, not unlike libGDX or MonoGame.
//!
//! It's opinionated, but small in scope and is intended to form a solid 'library'-like toolkit
//! for constructing small but fast 2d games (and maybe 3d someday). A lot of the work is left
//! to the author as to how they'd like to glue things together.

#![allow(incomplete_features)]

#![feature(downcast_unchecked)]
#![feature(generic_const_exprs)]
#![feature(const_refs_to_cell)]
#![feature(decl_macro)]

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

/// An context for object-based operations.
///
/// This context can be easily passed around the application and
/// allows resources to refer back to the originating server `S`.
pub struct Context<S: ?Sized>(std::rc::Rc<S>);

impl<S: ?Sized> Clone for Context<S> {
  /// Clones the reference to the core system.
  fn clone(&self) -> Self {
    Self(self.0.clone())
  }
}

impl<S: ?Sized> std::ops::Deref for Context<S> {
  type Target = S;

  /// Directly de-references the core system.
  fn deref(&self) -> &Self::Target {
    self.0.deref()
  }
}