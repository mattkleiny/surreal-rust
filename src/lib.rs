//! Packaging project for the Surreal engine.
//!
//! Importing this crate and enabling specific features will turn various functionalities on/off.

pub extern crate surreal_core as core;
#[cfg(feature = "editor")]
pub extern crate surreal_editor as editor;
#[cfg(feature = "csg")]
pub extern crate surreal_modules_csg as csg;
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
  #[cfg(feature = "gdscript")]
  pub use gdscript::*;
  #[cfg(feature = "prototype")]
  pub use prototype::*;
  #[cfg(feature = "voxels")]
  pub use voxels::*;
}
