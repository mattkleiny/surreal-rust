//! Root project for the Surreal engine.
//!
//! Surreal is a lightweight game engine built with Rust. It offers an
//! opinionated set of basic functionalities that game developers require to
//! get-things-done, without over-emphasising non goals such as performance or
//! feature-completeness.
//!
//! The engine is built on top of a number of core libraries, which are all
//! available as separate crates. This root project can be added as a
//! dependency, and features enabled/disabled to toggle the set of requirements
//! needed for any particular project.

#[cfg(feature = "ai")]
pub extern crate ai;
#[cfg(feature = "analytics")]
pub extern crate analytics;
#[cfg(feature = "animation")]
pub extern crate animation;
pub extern crate core;
#[cfg(feature = "csg")]
pub extern crate csg;
#[cfg(feature = "editor")]
pub extern crate editor;
#[cfg(feature = "prototype")]
pub extern crate prototype;
#[cfg(feature = "scripting")]
pub extern crate scripting;
#[cfg(feature = "streaming")]
pub extern crate streaming;
#[cfg(feature = "voxels")]
pub extern crate voxels;

#[rustfmt::skip]
pub mod prelude {
  pub use core::assets::*;
  pub use core::audio::*;
  pub use core::collections::*;
  pub use core::diagnostics::*;
  pub use core::engine::*;
  pub use core::fibers::*;
  pub use core::graphics::*;
  pub use core::graphs::*;
  pub use core::input::*;
  pub use core::io::*;
  pub use core::macros::*;
  pub use core::maths::*;
  pub use core::physics::*;
  pub use core::scene::*;
  pub use core::ui::*;
  pub use core::utilities::*;
}
