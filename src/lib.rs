//! Root project for the Surreal engine.
//!
//! Surreal is a lightweight game engine built with Rust. It offers an opinionated set of basic
//! functionalities that game developers require to get-things-done, without over-emphasising non
//! goals such as performance or feature-completeness.
//!
//! The engine is built on top of a number of core libraries, which are all available as separate
//! crates. This root project can be added as a dependency, and features enabled/disabled to toggle
//! the set of requirements needed for any particular project.

#[cfg(feature = "ai")]
pub extern crate ai;
pub extern crate core;
#[cfg(feature = "csg")]
pub extern crate csg;
#[cfg(feature = "editor")]
pub extern crate editor;
#[cfg(feature = "macros")]
pub extern crate macros;
#[cfg(feature = "prototype")]
pub extern crate prototype;
#[cfg(feature = "scene2d")]
pub extern crate scene2d;
#[cfg(feature = "scene3d")]
pub extern crate scene3d;
#[cfg(feature = "scripting")]
pub extern crate scripting;
#[cfg(feature = "streaming")]
pub extern crate streaming;
#[cfg(feature = "voxels")]
pub extern crate voxels;

pub mod prelude {
  pub use core::assets::*;
  pub use core::audio::*;
  pub use core::collections::*;
  pub use core::diagnostics::*;
  pub use core::engine::*;
  pub use core::graphics::*;
  pub use core::input::*;
  pub use core::io::*;
  pub use core::maths::*;
  pub use core::scene::*;
  pub use core::scripting::*;
  pub use core::ui::*;
  pub use core::utilities::*;
}

pub mod servers {
  #[cfg(feature = "audio")]
  pub use audio::*;
  #[cfg(feature = "graphics")]
  pub use graphics::*;
  #[cfg(feature = "physics")]
  pub use physics::*;
}
