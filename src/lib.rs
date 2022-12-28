//! Root project for the Surreal engine.
//!
//! Surreal is a lightweight game engine built with Rust. It offers an opinionated set of basic
//! functionalities that game developers require to get-things-done, without over-emphasising non
//! goals such as performance or feature-completeness.
//!
//! The engine is built on top of a number of core libraries, which are all available as separate
//! crates. This root project can be added as a dependency, and features enabled/disabled to toggle
//! the set of requirements needed for any particular project.

pub extern crate surreal_core as core;
#[cfg(feature = "editor")]
pub extern crate surreal_editor as editor;
#[cfg(feature = "blueprints")]
pub extern crate surreal_modules_blueprints as blueprints;
#[cfg(feature = "csg")]
pub extern crate surreal_modules_csg as csg;
#[cfg(feature = "fsm")]
pub extern crate surreal_modules_fsm as fsm;
#[cfg(feature = "gdscript")]
pub extern crate surreal_modules_gdscript as gdscript;
#[cfg(feature = "prototype")]
pub extern crate surreal_modules_prototype as prototype;
#[cfg(feature = "voxels")]
pub extern crate surreal_modules_voxels as voxels;

pub use core::Result;

pub mod prelude {
  //! A prelude for the Surreal engine.
  //!
  //! Import this module to get convenient access to all engine features.

  #[cfg(feature = "blueprints")]
  pub use blueprints::*;
  pub use core::assets::*;
  pub use core::audio::*;
  pub use core::collections::*;
  pub use core::diagnostics::*;
  pub use core::engine::*;
  pub use core::graphics::*;
  pub use core::input::*;
  pub use core::io::*;
  pub use core::maths::*;
  pub use core::scripting::*;
  pub use core::ui::*;
  pub use core::utilities::*;
  #[cfg(feature = "csg")]
  pub use csg::*;
  #[cfg(feature = "editor")]
  pub use editor::*;
  #[cfg(feature = "fsm")]
  pub use fsm::*;
  #[cfg(feature = "gdscript")]
  pub use gdscript::*;
  #[cfg(feature = "prototype")]
  pub use prototype::*;
  #[cfg(feature = "voxels")]
  pub use voxels::*;
}
